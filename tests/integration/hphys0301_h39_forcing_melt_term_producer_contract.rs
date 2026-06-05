use serde_json::Value;
use std::fs;

const SC_SNOWFREEZE: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const SC_WATBAL: &str = "docs/specifications/science-contracts/contracts/SC-WATBAL-001.md";
const INDEX: &str = "docs/specifications/science-contracts/index.md";
const PACKAGE: &str =
    "docs/work-packages/20260605-hphys0301-h39-forcing-melt-term-producer-closure-001/package.md";
const PROMPT: &str = "docs/work-packages/20260605-hphys0301-h39-forcing-melt-term-producer-closure-001/prompts/active/20260605-hphys0301-h39-forcing-melt-term-producer-closure-001_kickoff_agent_prompt.md";
const RUNNER: &str = "docs/work-packages/20260605-hphys0301-h39-forcing-melt-term-producer-closure-001/artifacts/hphys0301_h39_forcing_release_lineage.py";
const LEDGER: &str = "docs/work-packages/20260605-hphys0301-h39-forcing-melt-term-producer-closure-001/artifacts/h39-forcing-release-lineage-ledger.json";

#[test]
fn hphys0301_contracts_reclassify_h39_raw_rain_to_release_lineage() {
    let snow = fs::read_to_string(SC_SNOWFREEZE).unwrap();
    let watbal = fs::read_to_string(SC_WATBAL).unwrap();
    let index = fs::read_to_string(INDEX).unwrap();

    assert!(
        snow.contains("INV-SNOWFREEZE-032")
            && snow.contains("baseline residual rain-on-snow")
            && snow.contains("snow_hourly_rain_released_sum_m + snow_post_winter_rain_m")
            && snow.contains("not raw `snow_hourly_rain_sum_m`")
            && snow.contains("observe tags whose call sites are absent")
    );
    assert!(
        watbal.contains("INV-WATBAL-076")
            && watbal.contains("valid comparison uses openWEPP released plus post-winter rain")
            && watbal.contains("WB17/WB18/WB19/WB13 compensation remains prohibited")
    );
    assert!(
        index.contains("HPHYS0301 registry note")
            && index.contains("SC-SNOWFREEZE-001#INV-SNOWFREEZE-032")
            && index.contains("SC-WATBAL-001#INV-WATBAL-076")
    );
}

#[test]
fn hphys0301_package_and_prompt_require_implementation_or_blocker() {
    let package = fs::read_to_string(PACKAGE).unwrap();
    let prompt = fs::read_to_string(PROMPT).unwrap();

    assert!(
        package.contains("implementation-or-blocker")
            && package.contains("Treating baseline observe tags as source-line authority")
            && package.contains("artifacts/h39-forcing-release-lineage-ledger.json")
    );
    assert!(
        prompt.contains("Execution mode: package-end-to-end (default)")
            && prompt.contains("Autonomy: execute package phases end-to-end")
            && prompt.contains("If a source-line openWEPP producer defect is proven")
            && prompt.contains("record the concrete blocker")
    );
}

#[test]
fn hphys0301_runner_and_ledger_block_raw_rain_production_edit() {
    let runner = fs::read_to_string(RUNNER).unwrap();
    assert!(
        runner.contains("snow_hourly_rain_released_sum_m")
            && runner.contains("snow_post_winter_rain_m")
            && runner.contains("H298_RAW_A")
            && runner.contains("h39-rain-release-lineage-reclassified-hold")
    );

    let ledger_text = fs::read_to_string(LEDGER).unwrap();
    let row: Value = serde_json::from_str(&ledger_text).unwrap();
    assert_eq!(
        row["hphys0301_route"].as_str(),
        Some("h39-rain-release-lineage-reclassified-hold")
    );
    assert_eq!(row["production_edit_authorized"].as_bool(), Some(false));
    assert_eq!(
        row["production_forcing_edit_authorized"].as_bool(),
        Some(false)
    );

    let raw_delta = row["baseline_minus_open_raw_rain_mm"].as_f64().unwrap();
    let release_delta = row["baseline_minus_open_released_plus_post_rain_mm"]
        .as_f64()
        .unwrap();
    assert!(raw_delta.abs() > 10.0);
    assert!(release_delta.abs() < 1.0);
    assert!(
        row["blocking_invariant"]
            .as_str()
            .unwrap()
            .contains("H298 observe tag sites")
    );
}
