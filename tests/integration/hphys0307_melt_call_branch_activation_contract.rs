use serde_json::Value;
use std::fs;

const PACKAGE: &str = "docs/work-packages/20260605-hphys0307-melt-call-branch-activation-lineage-closure-001/package.md";
const PROMPT: &str = "docs/work-packages/20260605-hphys0307-melt-call-branch-activation-lineage-closure-001/prompts/active/20260605-hphys0307-melt-call-branch-activation-lineage-closure-001_kickoff_agent_prompt.md";
const RUNNER: &str = "docs/work-packages/20260605-hphys0307-melt-call-branch-activation-lineage-closure-001/artifacts/hphys0307_melt_call_branch_activation.py";
const LEDGER: &str = "docs/work-packages/20260605-hphys0307-melt-call-branch-activation-lineage-closure-001/artifacts/melt-call-branch-activation-ledger.json";
const SOURCE_LINEAGE: &str = "docs/work-packages/20260605-hphys0307-melt-call-branch-activation-lineage-closure-001/artifacts/melt-call-branch-activation-source-lineage.md";
const SC_WATBAL: &str = "docs/specifications/science-contracts/contracts/SC-WATBAL-001.md";

#[test]
fn hphys0307_package_requires_source_lineage_before_edits() {
    let package = fs::read_to_string(PACKAGE).unwrap();
    let prompt = fs::read_to_string(PROMPT).unwrap();

    for required in [
        "winter.for",
        "snowd.for",
        "melt.for",
        "snow_hourly_melt_branch_active",
        "baseline-extra",
        "openWEPP",
        "no production kernel edits unless",
        "no external connectivity",
        "Autonomy: execute package phases end-to-end",
    ] {
        assert!(
            package.contains(required) || prompt.contains(required),
            "missing HPHYS0307 package/prompt requirement {required}"
        );
    }
}

#[test]
fn hphys0307_contract_authority_is_registered() {
    let sc_watbal = fs::read_to_string(SC_WATBAL).unwrap();

    assert!(
        sc_watbal.contains("INV-WATBAL-080")
            && sc_watbal.contains("HPHYS0307 melt-call branch activation")
            && sc_watbal.contains("baseline-extra-melt-call")
            && sc_watbal.contains("openwepp-extra-melt-call")
            && sc_watbal.contains("compute_simimpl29_melt_hour")
            && sc_watbal.contains("WB13/WB17/WB18/WB19/WB12 compensation remains invalid")
    );
}

#[test]
fn hphys0307_runner_preserves_prior_ledger_and_no_remote_actions() {
    let runner = fs::read_to_string(RUNNER).unwrap();

    assert!(
        runner.contains("20260605-hphys0306-baseline-melt-term-observe-semantics-closure-001")
            && runner.contains("branch-active-melt-term-ledger.json")
            && runner.contains("baseline-extra-melt-call")
            && runner.contains("openwepp-extra-melt-call")
            && runner.contains("same-hour-multi-source-hold")
            && runner.contains("production_edit_authorized")
            && runner.contains("winter.for")
            && runner.contains("snowd.for")
            && !runner.contains(" git push ")
            && !runner.contains("\"push\"")
    );
}

#[test]
fn hphys0307_source_lineage_cites_baseline_and_openwepp_predicates() {
    let source_lineage = fs::read_to_string(SOURCE_LINEAGE).unwrap();

    for required in [
        "/workdir/wepp-forest_260430_baseline/src/winter.for",
        "/workdir/wepp-forest_260430_baseline/src/snowd.for",
        "crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs",
        "call snowd",
        "call melt",
        "compute_simimpl29_melt_hour",
        "melt_branch_active = 1.0",
    ] {
        assert!(
            source_lineage.contains(required),
            "missing source-lineage token {required}"
        );
    }
}

#[test]
fn hphys0307_executed_ledger_is_classification_coherent() {
    let text = fs::read_to_string(LEDGER).unwrap();
    let ledger: Vec<Value> = serde_json::from_str(&text).unwrap();

    assert_eq!(ledger.len(), 9);
    let mut baseline_extra_rows = 0;
    let mut openwepp_extra_rows = 0;
    let mut same_hour_rows = 0;
    for row in ledger {
        assert_eq!(row["production_edit_authorized"].as_bool(), Some(false));
        assert_eq!(
            row["upstream_source_ledger"].as_str(),
            Some("HPHYS0306 branch-active melt-term ledger")
        );
        assert!(row["required_next_action"].as_str().is_some());
        assert!(
            row["route"].as_str().unwrap().ends_with("-hold"),
            "row route must remain HOLD until source-line-owned defect proof exists"
        );

        let baseline_extra = row["baseline_extra_count"].as_u64().unwrap();
        let openwepp_extra = row["openwepp_extra_count"].as_u64().unwrap();
        let classification = row["source_classification"].as_str().unwrap();
        if baseline_extra > 0 {
            baseline_extra_rows += 1;
            assert_eq!(classification, "baseline-extra-melt-call");
            assert_eq!(row["route"].as_str(), Some("baseline-extra-melt-call-hold"));
            assert_eq!(openwepp_extra, 0);
        } else if openwepp_extra > 0 {
            openwepp_extra_rows += 1;
            assert_eq!(classification, "openwepp-extra-melt-call");
            assert_eq!(row["route"].as_str(), Some("openwepp-extra-melt-call-hold"));
        } else if row["hillslope_id"].as_u64() == Some(39)
            && row["window"].as_str() == Some("first-abs-storage-ge-10mm")
        {
            same_hour_rows += 1;
            assert_eq!(
                classification,
                "matched-branch-active-same-hour-multi-source"
            );
            assert_eq!(row["route"].as_str(), Some("same-hour-multi-source-hold"));
        }
    }

    assert_eq!(baseline_extra_rows, 7);
    assert_eq!(openwepp_extra_rows, 1);
    assert_eq!(same_hour_rows, 1);
}
