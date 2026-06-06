use std::fs;
use std::path::Path;
use std::process::Command;

const PACKAGE: &str =
    "docs/work-packages/20260605-hphys0313-snowpack-settling-carry-recursion-closure-001";

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("failed to read {path}: {error}"))
}

#[test]
fn hphys0313_contract_authority_is_registered() {
    let snowfreeze = read("docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md");
    assert!(snowfreeze.contains("contract_version:"));
    assert!(snowfreeze.contains("INV-SNOWFREEZE-038"));
    assert!(
        snowfreeze.contains("HPHYS0313 split-route snowpack settling/carry recursion invariant")
    );
    assert!(snowfreeze.contains("full-precision pinned-baseline `wdayct`"));
    assert!(snowfreeze.contains("SC-WATBAL-001#INV-WATBAL-086"));

    let watbal = read("docs/specifications/science-contracts/contracts/SC-WATBAL-001.md");
    assert!(watbal.contains("contract_version:"));
    assert!(watbal.contains("INV-WATBAL-086"));
    assert!(watbal.contains("HPHYS0313 split-route snowpack water-balance invariant"));
    assert!(watbal.contains("rounded observe output"));
    assert!(watbal.contains("SC-SNOWFREEZE-001#INV-SNOWFREEZE-038"));
}

#[test]
fn hphys0313_package_is_autonomous_and_continuation_scoped() {
    let package = read(&format!("{PACKAGE}/package.md"));
    assert!(package.contains("Status:"));
    assert!(package.contains("HPHYS0312 required continuation"));
    assert!(package.contains("Contract-First Sequence"));
    assert!(package.contains("dual review"));
    assert!(package.contains("dual verification"));
    assert!(package.contains("/workdir/wepp-forest_260430_baseline/src/snowd.for"));
    assert!(package.contains("No production Rust kernel edits unless source-line"));

    let prompt = read(&format!(
        "{PACKAGE}/prompts/active/20260605-hphys0313-snowpack-settling-carry-recursion-closure-001_kickoff_agent_prompt.md"
    ));
    assert!(prompt.contains("Execution mode: package-end-to-end"));
    assert!(prompt.contains("Required reading"));
    assert!(prompt.contains("Autonomy:"));
    assert!(prompt.contains("no external connectivity"));
}

#[test]
fn hphys0313_runner_fails_closed_and_cites_required_sources() {
    let runner = read(&format!(
        "{PACKAGE}/artifacts/hphys0313_snowpack_settling_carry_recursion.py"
    ));
    for token in [
        "snowd.for:61-65",
        "snowd.for:122-139",
        "snowd.for:310-312",
        "03_kernel_support_00_support_helpers.rs:3872-3877",
        "SourceLineEvidenceError",
        "PairedEvidenceError",
        "fixed-baseline-settling-instrumentation.patch",
    ] {
        assert!(runner.contains(token), "runner missing token {token}");
    }
    assert!(runner.contains("H313_WDAY"));
    assert!(runner.contains("H313_PRE"));
    assert!(runner.contains("H313_SETF"));
    assert!(runner.contains("H313_RAW"));
    assert!(runner.contains("H313_POST"));
    assert!(runner.contains("H313_FINAL"));
    assert!(runner.contains("1pe24.16"));
}

#[test]
fn hphys0313_runner_negative_fixture_fails_closed_on_missing_source_line() {
    let status = Command::new(".venv/bin/python")
        .arg(format!(
            "{PACKAGE}/artifacts/hphys0313_snowpack_settling_carry_recursion.py"
        ))
        .arg("--self-test-missing-source-line")
        .status()
        .expect("failed to run HPHYS0313 negative fixture");
    assert!(
        !status.success(),
        "missing-source-line self-test must fail closed"
    );
}

#[test]
fn hphys0313_executed_ledger_is_complete_and_hold_gated() {
    let ledger_path = format!("{PACKAGE}/artifacts/snowpack-settling-carry-recursion-ledger.json");
    assert!(
        Path::new(&ledger_path).exists(),
        "required HPHYS0313 ledger artifact is missing"
    );
    let ledger = read(&ledger_path);
    assert!(ledger.contains("settling_reconstruction"));
    assert!(ledger.contains("recursive_scan"));
    assert!(ledger.contains("production_edit_authorized"));
    assert!(ledger.contains("false"));
    assert!(ledger.contains("hourly-snowfall-input-lineage-hold"));
    assert!(ledger.contains("hphys0312_candidate_material_after_high_precision"));
    assert!(ledger.contains("snowfall_input_lineage_candidate"));
    assert!(ledger.contains("inferred_driftf_plus_driftg_m"));
    assert!(ledger.contains("recursive-"));

    let value: serde_json::Value = serde_json::from_str(&ledger).expect("valid ledger json");
    let rows = value.as_array().expect("ledger is array");
    assert_eq!(rows.len(), 6);
    let represented: i64 = rows
        .iter()
        .map(|row| row["affected_hphys0309_rows"].as_i64().unwrap())
        .sum();
    assert_eq!(represented, 57);
    let authorized = rows
        .iter()
        .filter(|row| row["production_edit_authorized"].as_bool() == Some(true))
        .count();
    assert_eq!(authorized, 0);
    let settling = rows
        .iter()
        .filter(|row| row["source_hphys0312_route"] == "settling-depth-update-hold")
        .count();
    let recursion = rows
        .iter()
        .filter(|row| row["source_hphys0312_route"] == "year-start-inherited-state-hold")
        .count();
    assert_eq!(settling, 3);
    assert_eq!(recursion, 3);
    let snowfall_lineage = rows
        .iter()
        .filter(|row| row["hphys0313_route"] == "hourly-snowfall-input-lineage-hold")
        .count();
    let recursive_year_start = rows
        .iter()
        .filter(|row| row["hphys0313_route"] == "recursive-year-start-inherited-state-hold")
        .count();
    let cold_driftg = rows
        .iter()
        .filter(|row| row["hphys0313_route"] == "cold-driftg-addition-lineage-hold")
        .count();
    assert_eq!(snowfall_lineage, 3);
    assert_eq!(recursive_year_start, 3);
    assert_eq!(cold_driftg, 0);
}

#[test]
fn hphys0313_source_lineage_and_patch_artifacts_verify_requirements() {
    let lineage_path =
        format!("{PACKAGE}/artifacts/snowpack-settling-carry-recursion-source-lineage.md");
    assert!(
        Path::new(&lineage_path).exists(),
        "required HPHYS0313 source-lineage artifact is missing"
    );
    let lineage = read(&lineage_path);
    assert!(lineage.contains("snowd.for:61-65"));
    assert!(lineage.contains("snowd.for:122-139"));
    assert!(lineage.contains("snowd.for:145-146"));
    assert!(lineage.contains("snowd.for:166-172"));
    assert!(lineage.contains("snowd.for:310-312"));
    assert!(lineage.contains("03_kernel_support_00_support_helpers.rs:3872-3924"));

    let patch = read(&format!(
        "{PACKAGE}/artifacts/fixed-baseline-settling-instrumentation.patch"
    ));
    assert!(patch.contains("H313_WDAY"));
    assert!(patch.contains("H313_PRE"));
    assert!(patch.contains("H313_SETF"));
    assert!(patch.contains("H313_RAW"));
    assert!(patch.contains("H313_POST"));
    assert!(patch.contains("H313_FINAL"));
    assert!(patch.contains("1pe24.16"));
}
