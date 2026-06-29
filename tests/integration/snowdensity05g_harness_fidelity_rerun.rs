use std::fs;
use std::path::PathBuf;

use openwepp_runner::{CoeMeltModel, CoeMeltRequest, run_coe_melt_snowbench};
use serde_json::Value;

const CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const PACKAGE: &str =
    "docs/work-packages/20260626-snowdensity-05g-harness-fidelity-rerun-001/package.md";
const SNOWBENCH_COE: &str = "crates/openwepp-runner/src/hillslope/snowbench_coe_melt.rs";

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn snowdensity05g_contract_records_harness_fidelity_gate() {
    let contract = read(CONTRACT);
    for marker in [
        "contract_version: 110",
        "INV-SNOWFREEZE-057",
        "SNOWDENSITY-05G harness-fidelity rerun",
        "configured coniferous validation fixtures",
        "canopy_cover_fraction` near `0.9`",
        "PySnobal bridge inversion is like-for-like",
        "`NON-PROMOTION` disposition",
        "`robust_fail_count=9`",
        "OBL-SNOWFREEZE-P-032",
        "SNOWDENSITY-05G Harness Fidelity Rerun Addendum",
    ] {
        assert_contains(&contract, marker, CONTRACT);
    }
}

#[test]
fn snowdensity05g_coe_melt_replay_uses_configured_canopy_and_proven_shortwave() {
    let root = repo_root();
    let fixture = root.join("tests/fixtures/snotel_observed/snotel_css_lab_ca");
    let output = root.join("target/snowdensity05g_harness_fidelity_test/css_lab");
    let _ = fs::remove_dir_all(&output);

    run_coe_melt_snowbench(&CoeMeltRequest {
        run_dir: fixture,
        run_file: None,
        output_dir: output.clone(),
        model: CoeMeltModel::CoeShortwaveAlbedoV1,
    })
    .expect("failed to run representative diagnostic CoE melt snowbench");

    let summary: Value = serde_json::from_str(
        &fs::read_to_string(output.join("coe_melt_summary.json"))
            .expect("missing coe_melt_summary.json"),
    )
    .expect("invalid coe_melt_summary.json");
    assert_eq!(summary["schema"], "snowdensity05g-coe-melt-snowbench-v1");
    assert_eq!(
        summary["canopy_source"],
        "direct_production_day_input.growth_state_for_publication.cancov"
    );
    let cancov = summary["constants"]["canopy_cover_fraction"]
        .as_f64()
        .expect("canopy_cover_fraction");
    assert!(
        (cancov - 0.9).abs() < 1e-12,
        "CSS Lab coniferous canopy should replay at 0.9, observed {cancov}"
    );
    assert_eq!(
        summary["canopy_series_summary"]["day_count"],
        summary["day_count"]
    );
    assert!(
        summary["canopy_series_path"]
            .as_str()
            .expect("canopy_series_path")
            .ends_with("canopy_series.csv")
    );
    assert_eq!(
        summary["shortwave_source"],
        "pysnobal_bridge_inversion_of_openwepp_winter_hourly_rad_mj_m2"
    );
    assert_eq!(summary["shortwave_bridge_like_for_like"], true);
    assert!(
        summary["shortwave_bridge_identity"]
            .as_str()
            .expect("shortwave_bridge_identity")
            .contains("/ 0.8")
    );
}

#[test]
fn snowdensity05g_package_and_source_preserve_diagnostic_confinement() {
    let package = read(PACKAGE);
    for marker in [
        "No default activation.",
        "No production parser, runfile, CLI, or output-schema selector.",
        "No melt coefficient, albedo constant, or shared-radiation retuning.",
        "No density/pack physics change.",
    ] {
        assert_contains(&package, marker, PACKAGE);
    }

    let source = read(SNOWBENCH_COE);
    assert_not_contains(&source, "DEFAULT_CANOPY_COVER_FRACTION", SNOWBENCH_COE);
    assert_contains(&source, "read_canopy_series", SNOWBENCH_COE);
    assert_contains(
        &source,
        "shortwave_bridge_like_for_like: true",
        SNOWBENCH_COE,
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
