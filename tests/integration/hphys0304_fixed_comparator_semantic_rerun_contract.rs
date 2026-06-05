use serde_json::Value;
use std::fs;

const PACKAGE: &str = "docs/work-packages/20260605-hphys0304-fixed-comparator-semantic-rerun-continuation-001/package.md";
const PROMPT: &str = "docs/work-packages/20260605-hphys0304-fixed-comparator-semantic-rerun-continuation-001/prompts/active/20260605-hphys0304-fixed-comparator-semantic-rerun-continuation-001_kickoff_agent_prompt.md";
const RUNNER: &str = "docs/work-packages/20260605-hphys0304-fixed-comparator-semantic-rerun-continuation-001/artifacts/hphys0304_fixed_comparator_semantic_rerun.py";
const SUMMARY: &str = "docs/work-packages/20260605-hphys0304-fixed-comparator-semantic-rerun-continuation-001/artifacts/fixed-baseline-semantic-summary.json";
const RECLASSIFICATION: &str = "docs/work-packages/20260605-hphys0304-fixed-comparator-semantic-rerun-continuation-001/artifacts/snow-rm-window-reclassification.json";
const LEDGER: &str = "docs/work-packages/20260605-hphys0304-fixed-comparator-semantic-rerun-continuation-001/artifacts/fixed-baseline-semantic-suite-ledger.json";
const FOLLOW_ON_PACKAGE: &str =
    "docs/work-packages/20260605-hphys0305-paired-melt-term-state-instrumentation-001/package.md";
const FOLLOW_ON_PROMPT: &str = "docs/work-packages/20260605-hphys0305-paired-melt-term-state-instrumentation-001/prompts/active/20260605-hphys0305-paired-melt-term-state-instrumentation-001_kickoff_agent_prompt.md";

#[test]
fn hphys0304_package_executes_adr0016_continuation_order_without_production_edits() {
    let package = fs::read_to_string(PACKAGE).unwrap();
    let prompt = fs::read_to_string(PROMPT).unwrap();

    assert!(
        package.contains("ADR-0016 Required Continuation Order step 1")
            && package.contains("H1..H39 semantic")
            && package.contains("Reclassify H1/H7/H39 snow/`RM` target windows")
            && package.contains("HPHYS0305")
            && package.contains("Production openWEPP physics edits")
            && package.contains("HPHYS0302 production-edit `HOLD` remains explicit")
    );
    assert!(
        prompt.contains("Execution mode: package-end-to-end (default)")
            && prompt.contains("Autonomy: execute package phases end-to-end")
            && prompt.contains("no external")
            && prompt.contains("connectivity")
            && prompt.contains("no production forcing")
            && prompt.contains("no downstream compensation")
            && prompt.contains("snow/`RM` target windows")
    );
}

#[test]
fn hphys0304_runner_uses_fixed_baseline_and_requires_fresh_candidate_identity() {
    let runner = fs::read_to_string(RUNNER).unwrap();

    assert!(
        runner.contains("47ac4c32faeea81bb99081f955a14c38b815ef4d")
            && runner.contains("fixed-baseline-semantic-reports")
            && runner.contains("--candidate-year-offset")
            && runner.contains("YEAR_OFFSET = 2012")
            && runner.contains("runtime_source_paths_changed")
            && runner.contains("candidate_outputs_reused")
            && runner.contains("production_edit_authorized")
            && runner.contains("20260605-hphys0305-paired-melt-term-state-instrumentation-001")
            && !runner.contains(" git push ")
            && !runner.contains("\"push\"")
    );
}

#[test]
fn hphys0304_artifacts_and_follow_on_are_status_coherent_when_executed() {
    let follow_on_package = fs::read_to_string(FOLLOW_ON_PACKAGE).unwrap();
    let follow_on_prompt = fs::read_to_string(FOLLOW_ON_PROMPT).unwrap();
    assert!(
        follow_on_package.contains("Status\n\nQueued")
            && follow_on_package.contains("amelt")
            && follow_on_package.contains("dmelt")
            && follow_on_package.contains("snodpt")
            && follow_on_package.contains("densgt")
            && follow_on_package.contains("No downstream compensation")
    );
    assert!(
        follow_on_prompt.contains("Execution mode: package-end-to-end (default)")
            && follow_on_prompt.contains("Autonomy: execute package phases end-to-end")
            && follow_on_prompt.contains("contract-first sequencing")
            && follow_on_prompt.contains("canonicalize-and-proceed")
    );

    assert!(
        std::path::Path::new(SUMMARY).exists(),
        "executed HPHYS0304 package must include fixed-baseline semantic summary"
    );
    assert!(
        std::path::Path::new(RECLASSIFICATION).exists(),
        "executed HPHYS0304 package must include snow/RM reclassification"
    );
    assert!(
        std::path::Path::new(LEDGER).exists(),
        "executed HPHYS0304 package must include semantic suite ledger"
    );

    let summary_text = fs::read_to_string(SUMMARY).unwrap();
    let summary: Vec<Value> = serde_json::from_str(&summary_text).unwrap();
    let columns: Vec<&str> = summary
        .iter()
        .filter_map(|row| row["column"].as_str())
        .collect();
    for required in [
        "RM",
        "Snow-Water",
        "Total-Soil",
        "SoilWaterTotal",
        "Ep",
        "Dp",
        "Q",
        "latqcc",
    ] {
        assert!(columns.contains(&required), "missing column {required}");
    }

    let reclass_text = fs::read_to_string(RECLASSIFICATION).unwrap();
    let reclass: Vec<Value> = serde_json::from_str(&reclass_text).unwrap();
    assert_eq!(reclass.len(), 9);
    assert!(reclass.iter().all(|row| {
        row["production_edit_authorized"].as_bool() == Some(false)
            && row["required_next_package"].as_str()
                == Some("20260605-hphys0305-paired-melt-term-state-instrumentation-001")
            && row["adr0011_confidence_tier"]
                .as_str()
                .unwrap_or("")
                .contains("single-OFE daily WAT")
    }));

    let ledger_text = fs::read_to_string(LEDGER).unwrap();
    let ledger: Value = serde_json::from_str(&ledger_text).unwrap();
    assert_eq!(ledger["production_edit_authorized"].as_bool(), Some(false));
    assert_eq!(
        ledger["hphys0302_hold_carried_forward"].as_bool(),
        Some(true)
    );
    assert_eq!(
        ledger["candidate"]["candidate_outputs_reused"].as_bool(),
        Some(true)
    );
}
