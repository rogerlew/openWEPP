use std::fs;
use std::path::PathBuf;

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
        "contract_id: SC-SNOWFREEZE-001",
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
fn snowdensity10_3_1a_historical_coe_replay_is_rejected_by_stage3_cutover() {
    let output_dir = PathBuf::from("target/snowdensity10_3_1a_contract/css_lab");
    let _ = fs::remove_dir_all(&output_dir);

    let error = run_coe_melt_snowbench(&CoeMeltRequest {
        run_dir: PathBuf::from("tests/fixtures/snotel_observed/snotel_css_lab_ca"),
        run_file: None,
        output_dir,
        model: CoeMeltModel::LegacyCoe,
    })
    .expect_err("historical CoE replay must not bypass the Stage3 V11 owner");
    let message = error.to_string();
    assert!(
        message.contains("snow.adaptive_stage3_legacy_sublimation_entry")
            && message.contains("outside [Some(0.0), Some(0.0)]"),
        "expected the typed Stage3 legacy-entry rejection, got {message}"
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
