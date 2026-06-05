use std::fs;

use serde_json::Value;

const SC_SNOWFREEZE: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const SC_WATBAL: &str = "docs/specifications/science-contracts/contracts/SC-WATBAL-001.md";
const PACKAGE: &str = "docs/work-packages/20260605-hphys0300-raw-hourly-melt-post-raw-routing-lineage-closure-001/package.md";
const PROMPT: &str = "docs/work-packages/20260605-hphys0300-raw-hourly-melt-post-raw-routing-lineage-closure-001/prompts/active/20260605-hphys0300-raw-hourly-melt-post-raw-routing-lineage-closure-001_kickoff_agent_prompt.md";
const RUNNER: &str = "docs/work-packages/20260605-hphys0300-raw-hourly-melt-post-raw-routing-lineage-closure-001/artifacts/hphys0300_raw_post_raw_lineage.py";
const LEDGER: &str = "docs/work-packages/20260605-hphys0300-raw-hourly-melt-post-raw-routing-lineage-closure-001/artifacts/raw-post-raw-lineage-ledger.json";

#[test]
fn hphys0300_contracts_require_term_state_evidence_before_melt_edits() {
    let snow = fs::read_to_string(SC_SNOWFREEZE).expect("snow contract should be readable");
    let watbal = fs::read_to_string(SC_WATBAL).expect("watbal contract should be readable");

    assert!(
        snow.contains("INV-SNOWFREEZE-031")
            && snow.contains("raw hourly melt/post-raw routing lineage invariant")
            && snow.contains("corrected HPHYS0299 depth-vs-depth forcing status")
            && snow.contains("`amelt`, `bmelt`, `cmelt`, `dmelt`")
            && snow.contains("`snodpt`, and `densgt`")
            && snow.contains("baseline_negative_raw_melt_sum_mm = 0.0")
            && snow.contains("H39 first-2013 remains a corrected-depth hourly-forcing seam")
            && snow.contains("Sufficiency and forcing-function closure")
            && snow.contains("must not request a further")
            && snow.contains("diagnostic-only package")
            && snow.contains("does not wait on")
            && snow.contains("raw-melt term instrumentation")
            && snow.contains("Aggregate `RM`, `Snow-Water`, or storage improvement alone"),
        "SC-SNOWFREEZE must gate HPHYS0300 raw/post-raw correction on term/state evidence"
    );
    assert!(
        watbal.contains("INV-WATBAL-075")
            && watbal.contains("raw hourly melt/post-raw routing water-balance invariant")
            && watbal.contains("full H1..H39 same-HEAD metrics")
            && watbal.contains("require term/state lineage evidence before snow producer edits")
            && watbal.contains("prohibit compensation through `Ep`, aggregate storage, percolation, lateral flow, or WB13 publication")
            && watbal.contains("Rows lacking term/state evidence remain `HOLD`")
            && watbal.contains("The HPHYS0300 evidence gate is bounded")
            && watbal.contains("must either implement the baseline-authoritative producer correction")
            && watbal.contains("another diagnostic-only package"),
        "SC-WATBAL must prohibit downstream compensation until raw/post-raw producer closure"
    );
}

#[test]
fn hphys0300_package_and_prompt_are_autonomous_and_corrected_ledger_scoped() {
    let package = fs::read_to_string(PACKAGE).expect("package should be readable");
    let prompt = fs::read_to_string(PROMPT).expect("prompt should be readable");

    assert!(
        package.contains("HPHYS0299 corrected the `hrsnow` unit/provenance seam")
            && package.contains("seven target windows to `raw-hourly-melt`")
            && package.contains("H7 first-2013")
            && package.contains("H39 first-2013")
            && package.contains("artifacts/raw-post-raw-lineage-ledger.json")
            && package.contains("Production edits, if any, are justified by term/state evidence"),
        "package must scope continuation from corrected HPHYS0299 evidence"
    );
    assert!(
        prompt.contains("Execution mode: package-end-to-end (default)")
            && prompt.contains("Autonomy: execute package phases end-to-end")
            && prompt.contains("/workdir/wepp-forest_260430_baseline")
            && prompt.contains(
                "no production raw-melt or routed-melt edits from aggregate deltas alone"
            )
            && prompt.contains("preserve corrected negative-melt authority"),
        "prompt must be autonomous and prohibit aggregate-delta production edits"
    );
}

#[test]
fn hphys0300_runner_routes_special_cases_without_authorizing_edits() {
    let runner = fs::read_to_string(RUNNER).expect("runner should be readable");
    let ledger_text = fs::read_to_string(LEDGER).expect("ledger should be readable");
    let rows: Vec<Value> =
        serde_json::from_str(&ledger_text).expect("ledger should parse as an array of rows");

    assert!(
        runner.contains("corrected-partition-ledger.json")
            && runner.contains("raw-hourly-melt")
            && runner.contains("post-raw routed-melt")
            && runner.contains("term_state_evidence_status")
            && runner.contains("aggregate-only")
            && runner.contains("production_edit_authorized")
            && runner.contains("baseline_negative_raw_melt_sum_mm")
            && runner.contains("corrected-depth-hourly-forcing-hold")
            && runner.contains("post-raw-routing-without-baseline-negative-melt-hold"),
        "runner must consume corrected HPHYS0299 evidence and keep unsupported edits unauthorized"
    );

    assert_eq!(
        rows.len(),
        9,
        "ledger must cover all nine H1/H7/H39 windows"
    );

    let route_count = |route: &str| {
        rows.iter()
            .filter(|row| row["hphys0300_route"].as_str() == Some(route))
            .count()
    };
    assert_eq!(route_count("raw-hourly-melt-term-state-hold"), 7);
    assert_eq!(
        route_count("post-raw-routing-without-baseline-negative-melt-hold"),
        1
    );
    assert_eq!(route_count("corrected-depth-hourly-forcing-hold"), 1);
    assert!(
        rows.iter()
            .all(|row| row["production_edit_authorized"].as_bool() == Some(false)),
        "aggregate-only evidence must not authorize production edits"
    );
    assert!(
        rows.iter()
            .all(|row| row["term_state_evidence_status"].as_str() == Some("aggregate-only")),
        "all rows must remain term/state holds until paired instrumentation exists"
    );

    let h7_first = rows
        .iter()
        .find(|row| {
            row["hillslope_id"].as_i64() == Some(7)
                && row["window"].as_str() == Some("first-abs-storage-ge-10mm")
        })
        .expect("H7 first-2013 row should exist");
    assert_eq!(
        h7_first["hphys0300_route"].as_str(),
        Some("post-raw-routing-without-baseline-negative-melt-hold")
    );
    let h7_baseline_negative_raw_melt = h7_first["baseline_negative_raw_melt_sum_mm"]
        .as_f64()
        .expect("H7 baseline negative raw melt should be numeric");
    assert!(h7_baseline_negative_raw_melt.abs() <= f64::EPSILON);

    let h39_first = rows
        .iter()
        .find(|row| {
            row["hillslope_id"].as_i64() == Some(39)
                && row["window"].as_str() == Some("first-abs-storage-ge-10mm")
        })
        .expect("H39 first-2013 row should exist");
    assert_eq!(
        h39_first["hphys0300_route"].as_str(),
        Some("corrected-depth-hourly-forcing-hold")
    );
}
