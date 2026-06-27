use std::fs;
use std::path::PathBuf;

use openwepp_runner::{CoeMeltModel, CoeMeltRequest, run_coe_melt_snowbench};
use serde_json::Value;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

#[test]
fn coe_melt_snowbench_runs_both_models_as_diagnostic_only() {
    let root = repo_root();
    let fixture = root.join("tests/fixtures/snotel_observed/snotel_css_lab_ca");
    let output_root = root.join("target/snowdensity05e_melt_adjudication_test");
    let _ = fs::remove_dir_all(&output_root);

    for (model_name, model) in [
        ("legacy_coe", CoeMeltModel::LegacyCoe),
        (
            "coe_shortwave_albedo_v1",
            CoeMeltModel::CoeShortwaveAlbedoV1,
        ),
    ] {
        let output = output_root.join(model_name);
        run_coe_melt_snowbench(&CoeMeltRequest {
            run_dir: fixture.clone(),
            run_file: None,
            output_dir: output.clone(),
            model,
        })
        .expect("failed to run diagnostic CoE melt snowbench");

        let summary: Value = serde_json::from_str(
            &fs::read_to_string(output.join("coe_melt_summary.json"))
                .expect("missing coe_melt_summary.json"),
        )
        .expect("invalid coe_melt_summary.json");
        assert_eq!(summary["schema"], "snowdensity05g-coe-melt-snowbench-v1");
        assert_eq!(summary["model_id"], model_name);
        assert_eq!(summary["no_site_constants"], true);
        assert!(
            summary["runtime_coupling"]
                .as_str()
                .expect("runtime_coupling string")
                .contains("no production activation")
        );
        assert!(
            summary["day_count"].as_u64().expect("day_count") > 0,
            "day_count must be positive"
        );
        assert!(
            summary["summary"]["total_snow_input_m"]
                .as_f64()
                .expect("total_snow_input_m")
                > 0.0,
            "fixture should contain snow input"
        );

        let csv = fs::read_to_string(output.join("coe_melt_snow.csv"))
            .expect("missing coe_melt_snow.csv");
        assert!(csv.starts_with(
            "date,snow_water_before_m,snow_input_m,rain_input_m,rain_retained_m,rain_released_m,"
        ));
        assert!(csv.contains(
            "snowpack_swe_balance_residual_m,routed_state_loss_residual_m,state_loss_available_storage_margin_m"
        ));
        assert!(
            csv.lines().count() > 100,
            "diagnostic snow series should contain many daily rows"
        );
    }
}

#[test]
fn coe_melt_adjudication_tool_is_documented_as_non_activation() {
    let root = repo_root();
    let script =
        fs::read_to_string(root.join("tools/snowfreeze_observed/coe_melt_adjudication.py"))
            .expect("missing coe_melt_adjudication.py");
    assert!(script.contains("diagnostic snowbench"));
    assert!(script.contains("does not activate the opt-in model in production runtime"));
    assert!(script.contains("legacy_coe"));
    assert!(script.contains("coe_shortwave_albedo_v1"));
    assert!(script.contains("SNOWDENSITY-05G"));
}
