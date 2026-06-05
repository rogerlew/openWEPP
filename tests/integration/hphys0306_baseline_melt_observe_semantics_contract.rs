use serde_json::Value;
use std::fs;

const PACKAGE: &str = "docs/work-packages/20260605-hphys0306-baseline-melt-term-observe-semantics-closure-001/package.md";
const PROMPT: &str = "docs/work-packages/20260605-hphys0306-baseline-melt-term-observe-semantics-closure-001/prompts/active/20260605-hphys0306-baseline-melt-term-observe-semantics-closure-001_kickoff_agent_prompt.md";
const RUNNER: &str = "docs/work-packages/20260605-hphys0306-baseline-melt-term-observe-semantics-closure-001/artifacts/hphys0306_branch_active_observe_semantics.py";
const LEDGER: &str = "docs/work-packages/20260605-hphys0306-baseline-melt-term-observe-semantics-closure-001/artifacts/branch-active-melt-term-ledger.json";
const SC_WATBAL: &str = "docs/specifications/science-contracts/contracts/SC-WATBAL-001.md";

#[test]
fn hphys0306_package_requires_branch_active_semantics() {
    let package = fs::read_to_string(PACKAGE).unwrap();
    let prompt = fs::read_to_string(PROMPT).unwrap();

    for required in [
        "branch-active",
        "inactive",
        "snow_hourly_melt_branch_active",
        "melt.for",
        "inactive-hour zero imputation",
        "no production physics edits",
    ] {
        assert!(
            package.contains(required) || prompt.contains(required),
            "missing HPHYS0306 package/prompt requirement {required}"
        );
    }
    assert!(
        prompt.contains("Execution mode: package-end-to-end (default)")
            && prompt.contains("Autonomy: execute package phases end-to-end")
            && prompt.contains("no external")
            && prompt.contains("connectivity")
    );
}

#[test]
fn hphys0306_contract_authority_is_registered() {
    let sc_watbal = fs::read_to_string(SC_WATBAL).unwrap();

    assert!(
        sc_watbal.contains("INV-WATBAL-079")
            && sc_watbal.contains("HPHYS0306 branch-active")
            && sc_watbal.contains("branch-active-mask-hold")
            && sc_watbal
                .contains("Inactive hours with no baseline melt call are not implicit zero-valued")
            && sc_watbal.contains("snow_hourly_melt_branch_active")
    );
}

#[test]
fn hphys0306_runner_preserves_fixed_comparator_and_no_remote_actions() {
    let runner = fs::read_to_string(RUNNER).unwrap();

    assert!(
        runner.contains("47ac4c32faeea81bb99081f955a14c38b815ef4d")
            && runner.contains("baseline_active_keys")
            && runner.contains("openwepp_active_keys")
            && runner.contains("snow_hourly_melt_branch_active")
            && runner.contains("TRACE_AUTHORITY_BOUNDARY = \"post_wb13\"")
            && runner.contains("branch-active-mask-hold")
            && runner.contains("branch-active-parser-conflict-hold")
            && runner.contains("same-hour-multi-source")
            && runner.contains("production_edit_authorized")
            && runner.contains("zero-impute")
            && !runner.contains(" git push ")
            && !runner.contains("\"push\"")
    );
}

#[test]
fn hphys0306_executed_ledger_is_status_coherent() {
    let text = fs::read_to_string(LEDGER).unwrap();
    let ledger: Vec<Value> = serde_json::from_str(&text).unwrap();

    assert_eq!(ledger.len(), 9);
    for row in ledger {
        assert_eq!(row["production_edit_authorized"].as_bool(), Some(false));
        assert!(row["baseline_active_count"].as_u64().unwrap() > 0);
        assert!(row["openwepp_active_count"].as_u64().unwrap() > 0);
        assert_eq!(row["branch_active_conflict_count"].as_u64(), Some(0));
        assert_eq!(row["trace_authority_boundary"].as_str(), Some("post_wb13"));
        assert_ne!(row["route"].as_str(), Some("paired-surface-gap"));
        assert!(row["first_divergent_source"].as_str().is_some());

        let baseline_only = row["baseline_only_active_count"].as_u64().unwrap();
        let openwepp_only = row["openwepp_only_active_count"].as_u64().unwrap();
        if baseline_only > 0 || openwepp_only > 0 {
            assert_eq!(
                row["branch_active_status"].as_str(),
                Some("branch-active-mask-gap")
            );
            assert_eq!(row["route"].as_str(), Some("branch-active-mask-hold"));
            assert_eq!(
                row["first_divergent_source"].as_str(),
                Some("melt-call-mask")
            );
        }
        if row["branch_active_status"].as_str() == Some("branch-active-mask-closed") {
            assert_ne!(row["route"].as_str(), Some("branch-active-mask-hold"));
        }
        if row["hillslope_id"].as_u64() == Some(39)
            && row["window"].as_str() == Some("first-abs-storage-ge-10mm")
        {
            assert_eq!(
                row["first_divergent_source"].as_str(),
                Some("same-hour-multi-source:cmelt,snodpt")
            );
            assert_eq!(row["route"].as_str(), Some("same-hour-multi-source-hold"));
        }
    }
}
