use std::fs;
use std::path::PathBuf;

use openwepp_runner::{
    PhysicsBulkRequest, PhysicsBulkVariant, physics_bulk_constants_for_variant,
    run_physics_bulk_snowbench,
};

const CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const PACKAGE: &str =
    "docs/work-packages/20260626-snowdensity-06-density-compaction-001/package.md";

#[test]
fn snowdensity06_contract_records_density_only_gate() {
    let contract = read(CONTRACT);
    for marker in [
        "contract_version: 123",
        "INV-SNOWFREEZE-058",
        "SNOWDENSITY-06 density-only Anderson/SNOBAL compaction candidate",
        "`density_compaction_v1`",
        "baseline candidate melt constants unchanged",
        "density/densification robust-cell profile",
        "OBL-SNOWFREEZE-P-033",
        "SNOWDENSITY-06 Density Compaction Addendum",
    ] {
        assert_contains(&contract, marker, CONTRACT);
    }

    let package = read(PACKAGE);
    for marker in [
        "Keep mixed/deciduous low-canopy melt work deferred to SNOWDENSITY-05H.",
        "baseline candidate melt constants",
        "No melt coefficient, albedo constant, canopy, shared-radiation",
        "density/densification cell summaries",
    ] {
        assert_contains(&package, marker, PACKAGE);
    }
}

#[test]
fn density_compaction_variant_preserves_melt_and_changes_density_only() {
    assert_eq!(
        PhysicsBulkVariant::parse("density_compaction_v1").expect("variant"),
        PhysicsBulkVariant::DensityCompactionV1
    );
    let candidate = physics_bulk_constants_for_variant(PhysicsBulkVariant::CandidateV1);
    let density = physics_bulk_constants_for_variant(PhysicsBulkVariant::DensityCompactionV1);

    assert_close(
        density.positive_degree_melt_kg_m2_per_c_hour,
        candidate.positive_degree_melt_kg_m2_per_c_hour,
    );
    assert_close(
        density.solar_melt_efficiency,
        candidate.solar_melt_efficiency,
    );
    assert_close(
        density.subfreezing_cold_content_relaxation_per_hour,
        candidate.subfreezing_cold_content_relaxation_per_hour,
    );
    assert!(density.new_snow_density_base_kg_m3 > candidate.new_snow_density_base_kg_m3);
    assert!(density.new_snow_density_max_kg_m3 > candidate.new_snow_density_max_kg_m3);
    assert!(density.dry_compaction_multiplier > candidate.dry_compaction_multiplier);
    assert!(density.wet_compaction_multiplier > candidate.wet_compaction_multiplier);
    assert_close(density.ptm_rate_per_hour, 0.01);
    assert_close(density.poc_rate_per_hour, 0.026);
    assert_close(density.compaction_rate_cos_amplitude, 23.5);
    assert_close(density.compaction_rate_offset, 24.5);
}

#[test]
fn density_compaction_snowbench_runs_offline_with_closure() {
    let output_dir = PathBuf::from("target/snowdensity06_contract/css_lab");
    let _ = fs::remove_dir_all(&output_dir);

    let report = run_physics_bulk_snowbench(&PhysicsBulkRequest {
        run_dir: PathBuf::from("tests/fixtures/snotel_observed/snotel_css_lab_ca"),
        run_file: None,
        output_dir: output_dir.clone(),
        variant: PhysicsBulkVariant::DensityCompactionV1,
    })
    .expect("offline density_compaction_v1 snowbench should run");

    assert_eq!(report.variant, "density_compaction_v1");
    assert_eq!(report.model_id, "physics_bulk_density_compaction_v1");
    assert!(report.no_site_constants);
    assert_eq!(
        report.runtime_coupling,
        "none; offline snowbench candidate only"
    );
    assert!(report.summary.max_abs_mass_balance_residual_kg_m2 < 1.0e-8);
    assert!(report.summary.max_abs_cold_content_residual_j_m2 < 1.0e-5);

    let summary = fs::read_to_string(output_dir.join("physics_bulk_summary.json"))
        .expect("summary json should exist");
    assert_contains(
        &summary,
        "\"ptm_rate_per_hour\"",
        "physics_bulk_summary.json",
    );
    assert_contains(
        &summary,
        "\"poc_rate_per_hour\"",
        "physics_bulk_summary.json",
    );
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

fn assert_close(actual: f64, expected: f64) {
    assert!(
        (actual - expected).abs() < 1.0e-12,
        "expected {actual} to equal {expected}"
    );
}
