use serde_json::Value;
use std::fs;

const SC_SNOWFREEZE: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const SC_WATBAL: &str = "docs/specifications/science-contracts/contracts/SC-WATBAL-001.md";
const INDEX: &str = "docs/specifications/science-contracts/index.md";
const PACKAGE: &str =
    "docs/work-packages/20260605-hphys0302-comparator-surface-audit-closure-001/package.md";
const PROMPT: &str = "docs/work-packages/20260605-hphys0302-comparator-surface-audit-closure-001/prompts/active/20260605-hphys0302-comparator-surface-audit-closure-001_kickoff_agent_prompt.md";
const RUNNER: &str = "docs/work-packages/20260605-hphys0302-comparator-surface-audit-closure-001/artifacts/hphys0302_comparator_surface_audit.py";
const LEDGER: &str = "docs/work-packages/20260605-hphys0302-comparator-surface-audit-closure-001/artifacts/comparator-surface-audit-ledger.json";

#[test]
fn hphys0302_contracts_require_same_surface_before_defect_claims() {
    let snow = fs::read_to_string(SC_SNOWFREEZE).unwrap();
    let watbal = fs::read_to_string(SC_WATBAL).unwrap();
    let index = fs::read_to_string(INDEX).unwrap();

    assert!(
        snow.contains("INV-SNOWFREEZE-033")
            && snow.contains("same physical quantity in the same units")
            && snow.contains("WB13/WAT `RM` and `Snow-Water`")
            && snow.contains("raw `hrmlt` and post-raw `wmelt`")
            && snow.contains("`amelt`, `bmelt`, `cmelt`, `dmelt`")
            && snow.contains("Observe tags or ledgers without source call sites")
    );
    assert!(
        watbal.contains("INV-WATBAL-077")
            && watbal.contains("publication/output-surface evidence only")
            && watbal.contains("Aggregate deltas without paired term/state surfaces remain `HOLD`")
            && watbal.contains("downstream water-balance compensation remains prohibited")
    );
    assert!(
        index.contains("HPHYS0302 registry note")
            && index.contains("SC-SNOWFREEZE-001#INV-SNOWFREEZE-033")
            && index.contains("SC-WATBAL-001#INV-WATBAL-077")
    );
}

#[test]
fn hphys0302_package_and_prompt_prohibit_aggregate_delta_edits() {
    let package = fs::read_to_string(PACKAGE).unwrap();
    let prompt = fs::read_to_string(PROMPT).unwrap();

    assert!(
        package.contains("same physical quantity in the same units")
            && package.contains("Production physics edits")
            && package.contains("term-level `amelt`/`bmelt`/`cmelt`/`dmelt`")
            && package.contains("Complete dual review/disposition and dual verification")
    );
    assert!(
        prompt.contains("Execution mode: package-end-to-end (default)")
            && prompt.contains("Autonomy: execute package phases end-to-end")
            && prompt.contains("no production forcing, snow, WB17, WB18, WB19, or WB13 edits from aggregate deltas alone")
            && prompt.contains("record the concrete missing surface blocker")
    );
}

#[test]
fn hphys0302_runner_and_ledger_hold_on_missing_term_surfaces() {
    let runner = fs::read_to_string(RUNNER).unwrap();
    assert!(
        runner.contains("same_physical_quantity_same_units")
            && runner.contains("blocked-missing-baseline-term-surface")
            && runner.contains("production_edit_authorized")
            && runner.contains("full-39-suite-metrics.md")
    );

    let ledger_text = fs::read_to_string(LEDGER).unwrap();
    let ledger: Value = serde_json::from_str(&ledger_text).unwrap();
    assert_eq!(ledger["production_edit_authorized"].as_bool(), Some(false));
    assert_eq!(
        ledger["decision"].as_str(),
        Some("hold-paired-baseline-melt-term-state-surface-missing")
    );
    assert_eq!(ledger["surface_counts"]["windows"].as_u64(), Some(9));
    assert_eq!(
        ledger["surface_counts"]["surfaces_per_window"].as_u64(),
        Some(5)
    );
    assert_eq!(ledger["surface_counts"]["total"].as_u64(), Some(45));
    assert_eq!(
        ledger["surface_counts"]["rm_like_for_like_pass"].as_u64(),
        Some(9)
    );
    assert_eq!(
        ledger["surface_counts"]["snow_water_output_surface_pass"].as_u64(),
        Some(9)
    );
    assert_eq!(
        ledger["surface_counts"]["aggregate_cutpoint_pass_not_term_authority"].as_u64(),
        Some(18)
    );
    assert_eq!(
        ledger["surface_counts"]["blocked_missing_baseline_term_surface"].as_u64(),
        Some(9)
    );

    let surface_summary = ledger["surface_summary"].as_array().unwrap();
    let verdict_for = |surface: &str| {
        surface_summary
            .iter()
            .find(|row| row["surface"].as_str() == Some(surface))
            .and_then(|row| row["verdict"].as_str())
            .unwrap()
    };
    assert_eq!(verdict_for("RM"), "like-for-like-pass");
    assert_eq!(verdict_for("Snow-Water"), "output-surface-pass");
    assert_eq!(
        verdict_for("melt_terms"),
        "blocked-missing-baseline-term-surface"
    );
}
