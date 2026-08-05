use std::fs;
use std::path::{Path, PathBuf};

use openwepp_runner::{CoeMeltModel, CoeMeltRequest, run_coe_melt_snowbench};

const CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const PACKAGE: &str =
    "docs/work-packages/20260626-snowdensity-10-3-1a-per-day-cancov-direct-runtime-001/package.md";
const SNOWBENCH: &str = "crates/openwepp-runner/src/hillslope/snowbench.rs";
const COE_MELT: &str = "crates/openwepp-runner/src/hillslope/snowbench_coe_melt.rs";
const DAY_INPUT_BUILDER: &str = "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs";

#[test]
fn snowdensity10_3_1a_contract_and_package_bind_daily_cancov() {
    let contract = read(CONTRACT);
    for marker in [
        "contract_version: 126",
        "cancov_daily_series",
        "INV-SNOWFREEZE-063",
        "OBL-SNOWFREEZE-P-038",
        "SNOWDENSITY-10.3.1a Per-Day Cancov Direct-Runtime Addendum",
        "scalar `primary_canopy_cover_fraction`",
    ] {
        assert_contains(&contract, marker, CONTRACT);
    }

    let package = read(PACKAGE);
    for marker in [
        "Per-Day Cancov Direct Runtime",
        "direct production growth-state path",
        "canopy_series.csv",
        "No melt, density, albedo, radiation, frost, or canopy tuning.",
    ] {
        assert_contains(&package, marker, PACKAGE);
    }
}

#[test]
fn snowdensity10_3_1a_coe_melt_consumes_daily_canopy_sidecar() {
    let output_dir = PathBuf::from("target/snowdensity10_3_1a_contract/css_lab");
    let _ = fs::remove_dir_all(&output_dir);

    let report = run_coe_melt_snowbench(&CoeMeltRequest {
        run_dir: PathBuf::from("tests/fixtures/snotel_observed/snotel_css_lab_ca"),
        run_file: None,
        output_dir: output_dir.clone(),
        model: CoeMeltModel::LegacyCoe,
    })
    .expect("CoE melt replay should consume direct-runtime canopy series");

    assert_eq!(
        report.canopy_source,
        "direct_production_day_input.growth_state_for_publication.cancov"
    );
    assert_eq!(report.canopy_series_summary.day_count, report.day_count);
    assert_close(
        report.constants.canopy_cover_fraction,
        report.canopy_series_summary.mean,
        1.0e-12,
    );
    assert!(
        Path::new(&report.canopy_series_path).is_file(),
        "canopy series sidecar should exist"
    );

    let canopy_series = read(&report.canopy_series_path);
    assert!(canopy_series.starts_with("date,day_index,canopy_cover_fraction,source\n"));
    assert_eq!(
        canopy_series
            .lines()
            .filter(|line| !line.trim().is_empty())
            .count()
            - 1,
        report.day_count
    );

    let coe_boundary = read(&output_dir.join("coe_melt_snow.csv").display().to_string());
    assert!(
        coe_boundary.starts_with(
            "date,snow_water_before_m,snow_input_m,rain_input_m,rain_retained_m,rain_released_m,liquid_holding_capacity_m,liquid_water_retained_m,liquid_water_released_m,snow_water_m,snow_depth_m,snow_density_kg_m3,raw_melt_m,gross_positive_generated_melt_m,redistributed_melt_m,routed_melt_m,snowpack_swe_loss_m,sublimation_m,snowpack_swe_balance_residual_m,routed_state_loss_residual_m,state_loss_available_storage_margin_m,snow_albedo,source\n"
        ),
        "CoE boundary CSV schema should include conservation and liquid-capacity operands"
    );
}

#[test]
fn snowdensity10_3_1a_source_uses_direct_runtime_series_not_scalar_replay() {
    let snowbench = read(SNOWBENCH);
    assert_contains(&snowbench, "build_direct_runtime_canopy_series", SNOWBENCH);
    assert_contains(
        &snowbench,
        "run_publication_capture_with_interleaved_day_inputs",
        SNOWBENCH,
    );
    let builder = read(DAY_INPUT_BUILDER);
    assert_contains(
        &builder,
        "day_input.canopy_cover_fraction = Some(growth_state_for_publication.canopy_cover_fraction)",
        DAY_INPUT_BUILDER,
    );

    let coe_melt = read(COE_MELT);
    assert_contains(&coe_melt, "read_canopy_series", COE_MELT);
    assert_contains(
        &coe_melt,
        "group_daily_forcing(hourly, &canopy_by_date)",
        COE_MELT,
    );
    assert_not_contains(
        &coe_melt,
        "group_daily_forcing(hourly, export_report.primary_canopy_cover_fraction)",
        COE_MELT,
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

fn assert_not_contains(text: &str, marker: &str, path: &str) {
    assert!(
        !text.contains(marker),
        "expected {path} not to contain marker: {marker}"
    );
}

fn assert_close(actual: f64, expected: f64, tolerance: f64) {
    let delta = (actual - expected).abs();
    assert!(
        delta <= tolerance,
        "expected {actual} ~= {expected} within {tolerance}, delta={delta}"
    );
}
