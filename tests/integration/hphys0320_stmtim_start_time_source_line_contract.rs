use std::fs;
use std::path::Path;

const PACKAGE: &str =
    "docs/work-packages/20260606-hphys0320-stmtim-start-time-source-line-closure-001";

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("failed to read {path}: {error}"))
}

fn assert_contains(content: &str, token: &str) {
    assert!(content.contains(token), "missing required token: {token}");
}

#[test]
fn hphys0320_contract_authority_is_registered() {
    let climate = read("docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md");
    assert_contains(&climate, "contract_version: 22");
    assert_contains(&climate, "INV-CLIMATE-018");
    assert_contains(&climate, "REF-CLIMATE-WF-WNTTIM-MIN");
    assert_contains(
        &climate,
        "HPHYS0320 SIMIMPL28 storm-start minimum-hour invariant",
    );
    assert_contains(&climate, "OBL-CLIMATE-P-013");
    assert_contains(&climate, "`wnttim < 1.0` normalizes to `1.0`");
    assert_contains(&climate, "derived 1-based storm-hour window convention");
    assert_contains(&climate, "SC-SNOWFREEZE-001#INV-SNOWFREEZE-046");
    assert_contains(&climate, "SC-WATBAL-001#INV-WATBAL-094");

    let snowfreeze = read("docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md");
    assert_contains(&snowfreeze, "contract_version: 55");
    assert_contains(&snowfreeze, "INV-SNOWFREEZE-046");
    assert_contains(&snowfreeze, "REF-SNOWFREEZE-LEGACY-WNTTIM-MIN");
    assert_contains(
        &snowfreeze,
        "HPHYS0320 `stmtim` start-time snow/freeze closure invariant",
    );
    assert_contains(&snowfreeze, "OBL-SNOWFREEZE-P-025");
    assert_contains(&snowfreeze, "combined `57` carried rows");
    assert_contains(&snowfreeze, "SC-CLIMATE-001#INV-CLIMATE-018");
    assert_contains(&snowfreeze, "SC-WATBAL-001#INV-WATBAL-094");

    let watbal = read("docs/specifications/science-contracts/contracts/SC-WATBAL-001.md");
    assert_contains(&watbal, "contract_version: 151");
    assert_contains(&watbal, "INV-WATBAL-094");
    assert_contains(&watbal, "HPHYS0320 `stmtim` start-time water-balance gate");
    assert_contains(&watbal, "OBL-WATBAL-P-030");
    assert_contains(&watbal, "WB13 `RM`/`Snow-Water`, WB17 ET, WB18 storage");
    assert_contains(&watbal, "SC-SNOWFREEZE-001#INV-SNOWFREEZE-046");
    assert_contains(&watbal, "SC-CLIMATE-001#INV-CLIMATE-018");

    let index = read("docs/specifications/science-contracts/index.md");
    assert_contains(&index, "SC-CLIMATE-001#INV-CLIMATE-018");
    assert_contains(&index, "SC-SNOWFREEZE-001#INV-SNOWFREEZE-046");
    assert_contains(&index, "SC-WATBAL-001#INV-WATBAL-094");
}

#[test]
fn hphys0320_package_scope_is_autonomous_and_right_sized() {
    let package = read(&format!("{PACKAGE}/package.md"));
    assert_contains(&package, "HPHYS0320 Stmtim Start-Time Source-Line Closure");
    assert_contains(&package, "contract-first");
    assert_contains(&package, "source-line classification");
    assert_contains(&package, "Regenerate H1/H7/H39 HPHYS0245 traces");
    assert_contains(&package, "Full carried-row disposition");
    assert_contains(&package, "combined `57` carried rows");

    let prompt = read(&format!(
        "{PACKAGE}/prompts/active/20260606-hphys0320-stmtim-start-time-source-line-closure-001_kickoff_agent_prompt.md"
    ));
    assert_contains(&prompt, "Execution mode: package-end-to-end");
    assert_contains(&prompt, "Required reading");
    assert_contains(&prompt, "Autonomy:");
    assert_contains(&prompt, "no external connectivity");
    assert_contains(&prompt, "source-line classify");
    assert_contains(&prompt, "dac3c950d8b16cc73774bf5ce2e7e11f80baac70");
}

#[test]
fn hphys0320_artifacts_publish_source_line_classification_and_disposition() {
    for relative in [
        "artifacts/source-line-classification.md",
        "artifacts/paired-trace-rerun-ledger.md",
        "artifacts/full-39-suite-metrics.md",
        "artifacts/kernel-profile-compliance-checklist.md",
        "artifacts/owned-file-manifest.md",
        "artifacts/gate-results.md",
        "artifacts/review-disposition.md",
        "artifacts/review_agent_a.md",
        "artifacts/review_agent_b.md",
        "artifacts/verification_agent_a.md",
        "artifacts/verification_agent_b.md",
        "artifacts/disposition.md",
        "artifacts/worker-handoff.md",
    ] {
        let path = format!("{PACKAGE}/{relative}");
        assert!(
            Path::new(&path).exists(),
            "missing required artifact: {relative}"
        );
        let content = read(&path);
        assert!(
            !content.contains("Status: queued"),
            "{relative} still has queued scaffold status"
        );
        assert!(
            !content.contains("Evidence mode: not-run"),
            "{relative} still has not-run evidence mode"
        );
    }

    let classification = read(&format!(
        "{PACKAGE}/artifacts/source-line-classification.md"
    ));
    assert_contains(
        &classification,
        "source_line_classification: `OPENWEPP-DEFECTIVE`",
    );
    assert_contains(&classification, "production_timing_edit_authorized: `true`");
    assert_contains(
        &classification,
        "/workdir/wepp-forest_260430_baseline/src/winter.for:206-235",
    );
    assert_contains(
        &classification,
        "/workdir/wepp-forest_260430_baseline/src/stmtim.for:43-64",
    );
    assert_contains(
        &classification,
        "crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs",
    );

    let rerun = read(&format!("{PACKAGE}/artifacts/paired-trace-rerun-ledger.md"));
    for token in [
        "wntdur = 11",
        "wnttim = 1",
        "active interval `1`",
        "snow branch `1`",
        "hrsnow ~= `0.00074545 m`",
        "carried_rows_closed_for_timing_seam: `57`",
    ] {
        assert_contains(&rerun, token);
    }

    let disposition = read(&format!("{PACKAGE}/artifacts/disposition.md"));
    assert_contains(&disposition, "Final disposition: `complete`");
    assert_contains(&disposition, "carried_rows_closed_for_timing_seam: `57`");
}
