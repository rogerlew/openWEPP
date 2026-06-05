use serde_json::Value;
use std::fs;

const PACKAGE: &str =
    "docs/work-packages/20260605-hphys0303-adr0016-fixed-comparator-ratification-001/package.md";
const PROMPT: &str = "docs/work-packages/20260605-hphys0303-adr0016-fixed-comparator-ratification-001/prompts/active/20260605-hphys0303-adr0016-fixed-comparator-ratification-001_kickoff_agent_prompt.md";
const RUNNER: &str = "docs/work-packages/20260605-hphys0303-adr0016-fixed-comparator-ratification-001/artifacts/hphys0303_adr0016_ratification.py";
const LEDGER: &str = "docs/work-packages/20260605-hphys0303-adr0016-fixed-comparator-ratification-001/artifacts/comparator-ratification-ledger.json";
const ADR0016: &str = "docs/decisions/0016-promote-260430-baseline-as-canonical-comparator-and-abandon-kernel-rewrite.md";
const ADR0012: &str = "docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md";

#[test]
fn hphys0303_package_requires_local_only_ratification_gates() {
    let package = fs::read_to_string(PACKAGE).unwrap();
    let prompt = fs::read_to_string(PROMPT).unwrap();

    assert!(
        package.contains("Pushing tags, branches, or commits to any remote")
            && package.contains("Fixed comparator branch/tag and exact commit SHA")
            && package.contains("Treating a partial local comparator build as ratified")
            && package.contains("HPHYS0302 production-edit `HOLD` remains explicit")
    );
    assert!(
        prompt.contains("Execution mode: package-end-to-end (default)")
            && prompt.contains("Autonomy: execute package phases end-to-end")
            && prompt.contains("no external connectivity")
            && prompt.contains("no silent defaults")
            && prompt.contains("no production forcing, snow, WB17, WB18, WB19, or WB13 edits")
    );
}

#[test]
fn hphys0303_runner_encodes_fixed_comparator_identity_without_remote_push() {
    let runner = fs::read_to_string(RUNNER).unwrap();
    assert!(
        runner.contains("wepp_260430_original_buggy_dac3c950")
            && runner.contains("kernel-rewrite-abandoned-20260605")
            && runner.contains("wepp_260430_negmeltfix_comparator")
            && runner.contains("03fee4558456535138592630b5dedc4d81ce8d06")
            && runner.contains("pstvML + ngtvML .le. 0.0")
            && runner.contains("1+ngtvML/pstvML")
            && runner.contains("\"remote_push_performed\": False")
            && runner.contains("EXPECTED_PARQUET_YEARS")
            && runner.contains("year_key_validation_pass")
            && runner.contains("SC unit/provenance lint did not pass")
            && runner.contains("smoke_checks_disposition")
            && !runner.contains(" git push ")
            && !runner.contains("\"push\"")
    );
}

#[test]
fn hphys0303_ledger_and_adrs_are_status_coherent() {
    let ledger_text = fs::read_to_string(LEDGER).unwrap();
    let ledger: Value = serde_json::from_str(&ledger_text).unwrap();
    let adr0016 = fs::read_to_string(ADR0016).unwrap();
    let adr0012 = fs::read_to_string(ADR0012).unwrap();

    assert_eq!(ledger["remote_push_performed"].as_bool(), Some(false));
    assert_eq!(ledger["production_edit_authorized"].as_bool(), Some(false));
    assert_eq!(
        ledger["hphys0302_hold_carried_forward"].as_bool(),
        Some(true)
    );
    assert_eq!(
        ledger["fixed_branch"].as_str(),
        Some("wepp_260430_negmeltfix_comparator")
    );
    assert!(
        ledger["ratification_checklist"]["hphys0302_hold"]["pass"]
            .as_bool()
            .unwrap_or(false)
    );

    let status = ledger["ratification_status"].as_str().unwrap();
    if status == "accepted-ready" {
        assert_eq!(ledger["blockers"].as_array().map(Vec::len), Some(0));
        assert!(
            ledger["sc_unit_provenance_lint"]["pass"]
                .as_bool()
                .unwrap_or(false),
            "accepted-ready requires passing SC lint"
        );
        assert!(
            ledger["ratification_checklist"]["sc_lint_result_artifact"]["pass"]
                .as_bool()
                .unwrap_or(false),
            "accepted-ready checklist must not pass on artifact presence alone"
        );
        assert!(
            ledger["fixed_baseline_parquets"]["year_key_validation_pass"]
                .as_bool()
                .unwrap_or(false),
            "accepted-ready requires H1..H39 parquet year/key validation"
        );
        assert_eq!(
            ledger["fixed_baseline_parquets"]["expected_years"]
                .as_array()
                .map(Vec::len),
            Some(4)
        );
        assert!(
            ledger["observe_identity"]["scope"]
                .as_str()
                .unwrap_or("")
                .contains("H1/H7/H39"),
            "observe identity evidence must be explicitly scoped"
        );
        assert!(
            adr0016.contains("H1/H7/H39")
                && !adr0016.contains("Re-run the H1..H39 semantic suite (openWEPP vs fixed baseline) and re-classify\n   the snow/`RM` windows under ADR-0011 confidence-tier rules."),
            "ADR-0016 must not overclaim unrun full semantic-suite reclassification as executed"
        );
        let smoke_disposition = ledger["smoke_checks_disposition"]["status"]
            .as_str()
            .unwrap_or("");
        let smoke_passed = ledger["smoke_checks"]
            .as_array()
            .map(|rows| {
                rows.iter()
                    .all(|row| row["pass"].as_bool().unwrap_or(false))
            })
            .unwrap_or(false);
        assert!(
            smoke_passed || smoke_disposition == "non_applicable_missing_helper_fixtures",
            "failed smoke checks require explicit non-applicable fixture disposition"
        );
        assert!(
            adr0016.contains("**Status:** Accepted"),
            "ADR-0016 must be Accepted"
        );
        assert!(
            !adr0016.contains("TBD"),
            "Accepted ADR-0016 must not retain TBD checklist entries"
        );
        assert!(
            adr0012.contains("wepp_260430 + negative-melt fix")
                && adr0012.contains(ledger["fixed_sha"].as_str().unwrap())
        );
    } else {
        assert_eq!(status, "proposed-hold");
        assert!(
            adr0016.contains("Proposed") && adr0016.contains("HPHYS0302's `HOLD` remains active")
        );
        assert!(
            ledger["blockers"].as_array().is_some(),
            "HOLD ledger must carry concrete blockers"
        );
    }
}
