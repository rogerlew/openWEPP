use std::fs;

use serde_json::Value;

const PACKAGE_DIR: &str =
    "docs/work-packages/20260605-hphys0309-snow-carry-depletion-lineage-closure-001";
const LEDGER_PATH: &str = "docs/work-packages/20260605-hphys0309-snow-carry-depletion-lineage-closure-001/artifacts/snow-carry-depletion-lineage-ledger.json";

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

#[test]
fn hphys0309_contract_authority_is_registered() {
    let snowfreeze = read("docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md");
    let watbal = read("docs/specifications/science-contracts/contracts/SC-WATBAL-001.md");

    assert!(
        snowfreeze.contains("contract_version:")
            && snowfreeze.contains("INV-SNOWFREEZE-034")
            && snowfreeze.contains("prior-day/hour carry state")
            && snowfreeze.contains("depletion lead hours")
            && snowfreeze.contains("REF-SNOWFREEZE-WEPPFOREST-WINTER-NEGMLT-FIX"),
        "SC-SNOWFREEZE-001 must register HPHYS0309 carry/depletion authority"
    );
    assert!(
        watbal.contains("contract_version:")
            && watbal.contains("INV-WATBAL-082")
            && watbal.contains("pre-day carry deficit")
            && watbal.contains("downstream compensation"),
        "SC-WATBAL-001 must register HPHYS0309 water-balance authority"
    );
}

#[test]
fn hphys0309_package_requires_carry_state_before_edits() {
    let package = read(&format!("{PACKAGE_DIR}/package.md"));
    let prompt = read(&format!(
        "{PACKAGE_DIR}/prompts/active/20260605-hphys0309-snow-carry-depletion-lineage-closure-001_kickoff_agent_prompt.md"
    ));

    for token in [
        "Contract-First Sequence",
        "fixed-comparator after-hour `snodpt`/`densgt`",
        "`snow_runtime_depth_before_m`",
        "No production Rust kernel edits",
        "WB13/WB17/WB18/WB19/WB12 compensation",
    ] {
        assert!(
            package.contains(token),
            "package must contain required carry/depletion token: {token}"
        );
    }
    assert!(
        prompt.contains("Execution mode: package-end-to-end")
            && prompt.contains("Autonomy:")
            && prompt.contains("no production edits before source-line proof"),
        "kickoff prompt must require end-to-end autonomous contract-first execution"
    );
}

#[test]
fn hphys0309_runner_uses_prior_ledgers_and_source_lines() {
    let runner = read(&format!(
        "{PACKAGE_DIR}/artifacts/hphys0309_snow_carry_depletion_lineage.py"
    ));

    for token in [
        "HPHYS0308_LEDGER",
        "baseline-observe-identity.json",
        "openwepp-trace-field-audit.json",
        "H305_S_OUT",
        "snow_runtime_depth_before_m",
        "snow_hourly_depth_after_m",
        "pre-day-carry-deficit-hold",
        "prior-day-openwepp-meltout-hold",
        "wepp_260430_negmeltfix_comparator commit 47ac4c32faeea81bb99081f955a14c38b815ef4d src/winter.for:434-453",
        "not-computable-baseline-no-same-day-zero",
        "production_edit_authorized",
    ] {
        assert!(
            runner.contains(token),
            "runner must include required carry/depletion token: {token}"
        );
    }
}

#[test]
fn hphys0309_source_lineage_cites_carry_publication() {
    let lineage = read(&format!(
        "{PACKAGE_DIR}/artifacts/snow-carry-depletion-lineage-source-lineage.md"
    ));

    for token in [
        "/workdir/wepp-forest_260430_baseline/src/snowd.for:50-53",
        "/workdir/wepp-forest_260430_baseline/src/snowd.for:303-312",
        "wepp_260430_negmeltfix_comparator_47ac4c32faee",
        "src/winter.for:434-453",
        "fixed-comparator-source-delta.patch",
        "03_kernel_support_00_support_helpers.rs:4111-4177",
        "03_kernel_support_00_support_helpers.rs:4231-4277",
    ] {
        assert!(
            lineage.contains(token),
            "source-lineage artifact must cite: {token}"
        );
    }
}

#[test]
fn hphys0309_executed_ledger_is_complete_and_hold_gated() {
    let ledger_text = read(LEDGER_PATH);
    let rows: Vec<Value> =
        serde_json::from_str(&ledger_text).expect("HPHYS0309 ledger must be valid JSON array");

    assert_eq!(
        rows.len(),
        58,
        "executed HPHYS0309 ledger must cover all 58 HPHYS0308 snow-state carry holds"
    );

    let mut pre_day_carry_deficit = 0;
    let mut prior_day_meltout = 0;
    let mut computed_lead = 0;
    let mut noncomputable_baseline_lead = 0;
    for row in rows {
        assert_eq!(
            row.get("hphys0308_route").and_then(Value::as_str),
            Some("snow-state-carry-depletion-hold"),
            "every HPHYS0309 row must derive from an HPHYS0308 snow-state hold"
        );
        assert_eq!(
            row.get("production_edit_authorized")
                .and_then(Value::as_bool),
            Some(false),
            "HPHYS0309 diagnostics must not authorize production edits"
        );
        assert!(
            row.get("day_start_depth_delta_openwepp_minus_baseline_m")
                .is_some(),
            "ledger row must include day-start depth delta"
        );
        assert!(
            row.get("openwepp_prior_hour_after_depth_m")
                .and_then(Value::as_f64)
                .is_some(),
            "ledger row must not synthesize missing openWEPP prior-hour depth"
        );
        assert!(
            row.get("openwepp_key_depth_after_m")
                .and_then(Value::as_f64)
                .is_some(),
            "ledger row must not synthesize missing openWEPP key depth"
        );
        assert!(
            row.get("openwepp_day_depth_after_m")
                .and_then(Value::as_object)
                .is_some_and(|hours| hours.len() == 24),
            "ledger row must include 24 explicit openWEPP hourly depth values"
        );
        match row.get("route").and_then(Value::as_str) {
            Some("pre-day-carry-deficit-hold") => pre_day_carry_deficit += 1,
            Some("prior-day-openwepp-meltout-hold") => prior_day_meltout += 1,
            other => panic!("unexpected HPHYS0309 route: {other:?}"),
        }
        match row
            .get("depletion_lead_evidence_state")
            .and_then(Value::as_str)
        {
            Some("computed") => computed_lead += 1,
            Some("not-computable-baseline-no-same-day-zero") => noncomputable_baseline_lead += 1,
            other => panic!("unexpected depletion lead evidence state: {other:?}"),
        }
    }
    assert_eq!(pre_day_carry_deficit, 45);
    assert_eq!(prior_day_meltout, 13);
    assert_eq!(computed_lead, 56);
    assert_eq!(noncomputable_baseline_lead, 2);
}
