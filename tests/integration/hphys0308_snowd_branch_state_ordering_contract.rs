use serde_json::Value;
use std::fs;

const PACKAGE: &str = "docs/work-packages/20260605-hphys0308-snowd-branch-predicate-state-ordering-closure-001/package.md";
const PROMPT: &str = "docs/work-packages/20260605-hphys0308-snowd-branch-predicate-state-ordering-closure-001/prompts/active/20260605-hphys0308-snowd-branch-predicate-state-ordering-closure-001_kickoff_agent_prompt.md";
const RUNNER: &str = "docs/work-packages/20260605-hphys0308-snowd-branch-predicate-state-ordering-closure-001/artifacts/hphys0308_snowd_branch_state_ordering.py";
const LEDGER: &str = "docs/work-packages/20260605-hphys0308-snowd-branch-predicate-state-ordering-closure-001/artifacts/snowd-branch-state-ordering-ledger.json";
const SOURCE_LINEAGE: &str = "docs/work-packages/20260605-hphys0308-snowd-branch-predicate-state-ordering-closure-001/artifacts/snowd-branch-state-ordering-source-lineage.md";
const SC_WATBAL: &str = "docs/specifications/science-contracts/contracts/SC-WATBAL-001.md";

#[test]
fn hphys0308_package_requires_key_level_state_ordering_before_edits() {
    let package = fs::read_to_string(PACKAGE).unwrap();
    let prompt = fs::read_to_string(PROMPT).unwrap();

    for required in [
        "snowd.for",
        "branch-extra",
        "state-ordering",
        "snow_hourly_depth_before_m",
        "snow_hourly_melt_branch_active",
        "no production kernel edits unless",
        "no external connectivity",
        "Autonomy: execute package phases end-to-end",
    ] {
        assert!(
            package.contains(required) || prompt.contains(required),
            "missing HPHYS0308 package/prompt requirement {required}"
        );
    }
}

#[test]
fn hphys0308_contract_authority_is_registered() {
    let sc_watbal = fs::read_to_string(SC_WATBAL).unwrap();

    assert!(
        sc_watbal.contains("contract_version:")
            && sc_watbal.contains("INV-WATBAL-081")
            && sc_watbal.contains("HPHYS0308 branch-extra state-ordering")
            && sc_watbal.contains("snow_hourly_depth_before_m")
            && sc_watbal.contains("snow-state-carry-depletion-hold")
            && sc_watbal.contains("WB13/WB17/WB18/WB19/WB12 compensation remains invalid")
    );
}

#[test]
fn hphys0308_runner_uses_prior_ledgers_without_remote_actions() {
    let runner = fs::read_to_string(RUNNER).unwrap();

    assert!(
        runner.contains("branch-active-melt-term-ledger.json")
            && runner.contains("melt-call-branch-activation-ledger.json")
            && runner.contains("baseline-observe-identity.json")
            && runner.contains("openwepp-trace-field-audit.json")
            && runner.contains("snow-state-carry-depletion-hold")
            && runner.contains("baseline-branch-instrumentation-hold")
            && runner.contains("production_edit_authorized")
            && !runner.contains(" git push ")
            && !runner.contains("\"push\"")
    );
}

#[test]
fn hphys0308_source_lineage_cites_predicate_and_state_surfaces() {
    let source_lineage = fs::read_to_string(SOURCE_LINEAGE).unwrap();

    for required in [
        "/workdir/wepp-forest_260430_baseline/src/snowd.for:116-193",
        "snodep .gt. 0.0",
        "call melt",
        "snow_hourly_depth_before_m",
        "snow_hourly_depth_available_m",
        "snow_hourly_depth_after_m",
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
fn hphys0308_executed_ledger_is_key_level_and_hold_gated() {
    let text = fs::read_to_string(LEDGER).unwrap();
    let ledger: Vec<Value> = serde_json::from_str(&text).unwrap();

    assert_eq!(ledger.len(), 59);
    let mut baseline_extra = 0;
    let mut openwepp_extra = 0;
    let mut snow_state_holds = 0;
    let mut baseline_instrumentation_holds = 0;

    for row in ledger {
        assert_eq!(row["production_edit_authorized"].as_bool(), Some(false));
        assert_eq!(
            row["fixed_comparator_commit"].as_str(),
            Some("47ac4c32faeea81bb99081f955a14c38b815ef4d")
        );
        assert!(row["route"].as_str().unwrap().ends_with("-hold"));

        match row["branch_extra_lane"].as_str().unwrap() {
            "baseline-extra-melt-call" => {
                baseline_extra += 1;
                assert_eq!(row["baseline_melt_call_observed"].as_bool(), Some(true));
                assert_eq!(row["openwepp_branch_active"].as_bool(), Some(false));
                assert_eq!(
                    row["route"].as_str(),
                    Some("snow-state-carry-depletion-hold")
                );
                snow_state_holds += 1;
            }
            "openwepp-extra-melt-call" => {
                openwepp_extra += 1;
                assert_eq!(row["baseline_melt_call_observed"].as_bool(), Some(false));
                assert_eq!(row["openwepp_branch_active"].as_bool(), Some(true));
                assert_eq!(
                    row["route"].as_str(),
                    Some("baseline-branch-instrumentation-hold")
                );
                baseline_instrumentation_holds += 1;
            }
            lane => panic!("unexpected branch-extra lane {lane}"),
        }
    }

    assert_eq!(baseline_extra, 58);
    assert_eq!(openwepp_extra, 1);
    assert_eq!(snow_state_holds, 58);
    assert_eq!(baseline_instrumentation_holds, 1);
}
