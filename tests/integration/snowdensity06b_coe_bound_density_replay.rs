use std::fs;
use std::path::{Path, PathBuf};

use openwepp_runner::{
    CoeBoundDensityRequest, CoeMeltModel, PhysicsBulkVariant, run_coe_bound_density_snowbench,
};

const CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const PACKAGE: &str =
    "docs/work-packages/20260626-snowdensity-06b-coe-bound-density-replay-001/package.md";

#[test]
fn snowdensity06b_contract_records_coe_bound_density_gate() {
    let contract = read(CONTRACT);
    for marker in [
        "contract_version: 99",
        "INV-SNOWFREEZE-059",
        "SNOWDENSITY-06B CoE-bound density replay",
        "preserve CoE `snow_water_m` identity",
        "OBL-SNOWFREEZE-P-034",
        "SNOWDENSITY-06B CoE-Bound Density Replay Addendum",
    ] {
        assert_contains(&contract, marker, CONTRACT);
    }

    let package = read(PACKAGE);
    for marker in [
        "CoE-bound density replay",
        "No mixed/deciduous low-canopy work",
        "SWE exactly on daily rows",
        "density/densification robust-cell evidence",
    ] {
        assert_contains(&package, marker, PACKAGE);
    }
}

#[test]
fn coe_bound_density_replay_preserves_coe_swe_and_changes_density_surface() {
    let output_dir = PathBuf::from("target/snowdensity06b_contract/css_lab_legacy");
    let _ = fs::remove_dir_all(&output_dir);

    let report = run_coe_bound_density_snowbench(&CoeBoundDensityRequest {
        run_dir: PathBuf::from("tests/fixtures/snotel_observed/snotel_css_lab_ca"),
        run_file: None,
        output_dir: output_dir.clone(),
        coe_model: CoeMeltModel::LegacyCoe,
        density_variant: PhysicsBulkVariant::DensityCompactionV1,
    })
    .expect("offline CoE-bound density replay should run");

    assert_eq!(
        report.schema,
        "snowdensity06b-coe-bound-density-snowbench-v1"
    );
    assert_eq!(report.coe_boundary_model, "legacy_coe");
    assert_eq!(report.density_variant, "density_compaction_v1");
    assert_eq!(
        report.runtime_coupling,
        "none; offline CoE-bound density replay only"
    );
    assert!(report.no_site_constants);
    assert!(report.summary.max_abs_coe_swe_identity_residual_m < 1.0e-12);

    let (replay_header, replay) = read_csv(&output_dir.join("coe_bound_density_snow.csv"));
    let (coe_header, coe) = read_csv(&output_dir.join("coe_boundary/coe_melt_snow.csv"));
    assert_eq!(replay.len(), coe.len());
    let mut density_difference_count = 0;
    for (replay_row, coe_row) in replay.iter().zip(&coe) {
        assert_eq!(
            csv_field(&replay_header, replay_row, "date"),
            csv_field(&coe_header, coe_row, "date"),
            "date alignment"
        );
        assert_close(
            parse(csv_field(&replay_header, replay_row, "snow_water_m")),
            parse(csv_field(&coe_header, coe_row, "snow_water_m")),
            1.0e-12,
        );
        if parse(csv_field(&coe_header, coe_row, "snow_water_m")) > 1.0e-9 {
            let density_delta =
                (parse(csv_field(&replay_header, replay_row, "snow_density_kg_m3"))
                    - parse(csv_field(&coe_header, coe_row, "snow_density_kg_m3")))
                .abs();
            if density_delta > 1.0e-9 {
                density_difference_count += 1;
            }
        }
    }
    assert!(
        density_difference_count > 0,
        "candidate density replay must not simply copy CoE density"
    );

    let summary = read(
        &output_dir
            .join("coe_bound_density_summary.json")
            .display()
            .to_string(),
    );
    assert_contains(
        &summary,
        "\"max_abs_coe_swe_identity_residual_m\"",
        "summary",
    );
    assert_contains(&summary, "\"ptm_rate_per_hour\"", "summary");
}

#[test]
fn coe_bound_density_rejects_non_density_compaction_variants() {
    let output_dir = PathBuf::from("target/snowdensity06b_contract/reject_variant");
    let _ = fs::remove_dir_all(&output_dir);

    let error = run_coe_bound_density_snowbench(&CoeBoundDensityRequest {
        run_dir: PathBuf::from("tests/fixtures/snotel_observed/snotel_css_lab_ca"),
        run_file: None,
        output_dir,
        coe_model: CoeMeltModel::LegacyCoe,
        density_variant: PhysicsBulkVariant::CandidateV1,
    })
    .expect_err("non-density-compaction variants must fail for 06B");

    assert_contains(
        &error.to_string(),
        "accepts only density_compaction_v1 or spring_densification_v1",
        "error",
    );
}

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn read_csv(path: &Path) -> (Vec<String>, Vec<Vec<String>>) {
    let text = fs::read_to_string(path)
        .unwrap_or_else(|err| panic!("failed to read {}: {err}", path.display()));
    let mut lines = text.lines();
    let header = lines
        .next()
        .unwrap_or_else(|| panic!("{} missing CSV header", path.display()))
        .split(',')
        .map(str::to_string)
        .collect::<Vec<_>>();
    let rows = lines
        .filter(|line| !line.trim().is_empty())
        .map(|line| line.split(',').map(str::to_string).collect())
        .collect();
    (header, rows)
}

fn csv_field<'a>(header: &[String], row: &'a [String], field: &str) -> &'a str {
    let index = header
        .iter()
        .position(|column| column == field)
        .unwrap_or_else(|| panic!("missing CSV field {field}"));
    row.get(index)
        .unwrap_or_else(|| panic!("row missing CSV field {field}"))
}

fn parse(value: &str) -> f64 {
    value
        .parse::<f64>()
        .unwrap_or_else(|err| panic!("failed to parse {value}: {err}"))
}

fn assert_contains(text: &str, marker: &str, path: &str) {
    assert!(
        text.contains(marker),
        "expected {path} to contain marker: {marker}"
    );
}

fn assert_close(actual: f64, expected: f64, tolerance: f64) {
    assert!(
        (actual - expected).abs() <= tolerance,
        "expected {actual} to equal {expected} within {tolerance}"
    );
}
