use std::fs;
use std::path::Path;

const PACKAGE: &str =
    "docs/work-packages/20260606-hphys0319-fixed-baseline-stmtim-observe-recovery-001";

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("failed to read {path}: {error}"))
}

fn assert_contains(content: &str, token: &str) {
    assert!(content.contains(token), "missing required token: {token}");
}

#[test]
fn hphys0319_contract_authority_is_registered() {
    let climate = read("docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md");
    assert_contains(&climate, "contract_version: 22");
    assert_contains(&climate, "INV-CLIMATE-017");
    assert_contains(
        &climate,
        "HPHYS0319 fixed-baseline `stmtim` observe recovery invariant",
    );
    assert_contains(&climate, "OBL-CLIMATE-P-012");
    assert_contains(&climate, "snow.hourly.stmtim.*_0011");
    assert_contains(&climate, "SC-SNOWFREEZE-001#INV-SNOWFREEZE-045");
    assert_contains(&climate, "SC-WATBAL-001#INV-WATBAL-093");

    let snowfreeze = read("docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md");
    assert_contains(&snowfreeze, "contract_version: 57");
    assert_contains(&snowfreeze, "INV-SNOWFREEZE-045");
    assert_contains(
        &snowfreeze,
        "HPHYS0319 fixed-baseline `stmtim` observe recovery invariant",
    );
    assert_contains(&snowfreeze, "OBL-SNOWFREEZE-P-024");
    assert_contains(&snowfreeze, "combined `57` carried rows");
    assert_contains(&snowfreeze, "SC-CLIMATE-001#INV-CLIMATE-017");
    assert_contains(&snowfreeze, "SC-WATBAL-001#INV-WATBAL-093");

    let watbal = read("docs/specifications/science-contracts/contracts/SC-WATBAL-001.md");
    assert_contains(&watbal, "contract_version: 152");
    assert_contains(&watbal, "INV-WATBAL-093");
    assert_contains(
        &watbal,
        "HPHYS0319 fixed-baseline `stmtim` observe water-balance gate",
    );
    assert_contains(&watbal, "OBL-WATBAL-P-029");
    assert_contains(&watbal, "source-line-owned OpenWEPP defect authority");
    assert_contains(&watbal, "SC-SNOWFREEZE-001#INV-SNOWFREEZE-045");
    assert_contains(&watbal, "SC-CLIMATE-001#INV-CLIMATE-017");

    let index = read("docs/specifications/science-contracts/index.md");
    assert_contains(&index, "SC-CLIMATE-001#INV-CLIMATE-017");
    assert_contains(&index, "SC-SNOWFREEZE-001#INV-SNOWFREEZE-045");
    assert_contains(&index, "SC-WATBAL-001#INV-WATBAL-093");
}

#[test]
fn hphys0319_package_is_autonomous_and_no_production_edit_scoped() {
    let package = read(&format!("{PACKAGE}/package.md"));
    assert_contains(&package, "Status: executed-hold");
    assert_contains(&package, "Contract-First Sequence");
    assert_contains(&package, "Temporary execution writes are limited");
    assert_contains(&package, "production edit authorization set to `false`");
    assert_contains(
        &package,
        "No production Rust precipitation-phase physics edit",
    );
    assert_contains(
        &package,
        "No permanent edit to `/workdir/wepp-forest_260430_baseline`",
    );
    assert_contains(&package, "Dual review findings");

    let prompt = read(&format!(
        "{PACKAGE}/prompts/active/20260606-hphys0319-fixed-baseline-stmtim-observe-recovery-001_kickoff_agent_prompt.md"
    ));
    assert_contains(&prompt, "Execution mode: package-end-to-end");
    assert_contains(&prompt, "Required reading");
    assert_contains(&prompt, "Autonomy:");
    assert_contains(&prompt, "no external connectivity");
    assert_contains(&prompt, "no production");
    assert_contains(&prompt, "dac3c950d8b16cc73774bf5ce2e7e11f80baac70");
}

#[test]
fn hphys0319_recovery_script_is_scoped_and_records_required_tags() {
    let script = read(&format!(
        "{PACKAGE}/artifacts/hphys0319_fixed_stmtim_observe.py"
    ));
    for token in [
        "/workdir/wepp-forest_260430_baseline",
        "dac3c950d8b16cc73774bf5ce2e7e11f80baac70",
        "/tmp/hphys0319_fixed_stmtim_observe_20260606T000000Z",
        "wepp_observe.on",
        "wepp_observe.log",
        "STM_RAIN_STMDUR",
        "STM_WNTDUR_WNTTIM",
        "STM_TEMP_RST",
        "STM_OUT_RAIN_SNOW",
        "STM_ACTIVE_BRANCH",
        "OPENWEPP_HPHYS0245_TRACE_PATH",
        "snow_hourly_stmtim_hrsnow_m",
        "production_physics_edit_authorized",
    ] {
        assert_contains(&script, token);
    }
}

#[test]
fn hphys0319_artifacts_publish_paired_observe_classification() {
    let ledger_path = format!("{PACKAGE}/artifacts/fixed-baseline-stmtim-observe-ledger.md");
    assert!(
        Path::new(&ledger_path).exists(),
        "required HPHYS0319 fixed-baseline observe ledger artifact is missing"
    );
    let ledger = read(&ledger_path);
    assert!(!ledger.contains("Status: queued"));
    assert!(!ledger.contains("Evidence mode: not-run"));
    assert_contains(&ledger, "Evidence mode: Ran");
    assert_contains(&ledger, "fixed_baseline_stmtim_observe_available: `true`");
    assert_contains(
        &ledger,
        "baseline_commit: `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`",
    );
    assert_contains(&ledger, "carried_rows_total: `57`");
    assert_contains(&ledger, "`STM_RAIN_STMDUR`");
    assert_contains(&ledger, "`STM_WNTDUR_WNTTIM`");
    assert_contains(&ledger, "`STM_TEMP_RST`");
    assert_contains(&ledger, "`STM_OUT_RAIN_SNOW`");
    assert_contains(&ledger, "`STM_ACTIVE_BRANCH`");
    for hill in ["H1", "H7", "H39"] {
        assert_contains(&ledger, hill);
    }

    let classification = read(&format!(
        "{PACKAGE}/artifacts/paired-stmtim-observe-classification.md"
    ));
    assert!(!classification.contains("Status: queued"));
    assert!(!classification.contains("Evidence mode: not-run"));
    assert_contains(&classification, "Evidence mode: Ran");
    assert_contains(
        &classification,
        "production_physics_edit_authorized: `false`",
    );
    assert_contains(&classification, "carried_rows_total: `57`");
    assert_contains(
        &classification,
        "paired_fixed_baseline_openwepp_stmtim_values: `true`",
    );
    assert_contains(&classification, "snow.hourly.stmtim.hrsnow_m_0011");
    assert_contains(&classification, "snow.hourly.stmtim.active_interval_0011");
    assert_contains(&classification, "HPHYS0320");
}

#[test]
fn hphys0319_artifacts_are_executed_and_gates_are_recorded() {
    for relative in [
        "artifacts/README.md",
        "artifacts/contract-implementation-evidence.md",
        "artifacts/contract-test-implementation-evidence.md",
        "artifacts/pre-implementation-contract-gate.md",
        "artifacts/implementation-test-evidence.md",
        "artifacts/fixed-baseline-stmtim-observe-ledger.md",
        "artifacts/paired-stmtim-observe-classification.md",
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
        "prompts/README.md",
        "prompts/active/README.md",
        "prompts/archived/README.md",
    ] {
        let content = read(&format!("{PACKAGE}/{relative}"));
        assert!(
            !content.contains("Status: queued"),
            "{relative} still has queued scaffold status"
        );
        assert!(
            !content.contains("Evidence mode: not-run"),
            "{relative} still has not-run evidence mode"
        );
    }

    let metrics = read(&format!("{PACKAGE}/artifacts/full-39-suite-metrics.md"));
    assert_contains(&metrics, "H1..H39");
    assert_contains(&metrics, "0/39");
    assert_contains(&metrics, "observe-recovery only carry-forward");

    let gate_results = read(&format!("{PACKAGE}/artifacts/gate-results.md"));
    for token in [
        "cargo fmt --check",
        "cargo clippy --workspace --all-targets -- -D warnings",
        "cargo test --workspace",
        "cargo deny check",
        "markdown-doc lint",
        "check_authority_suite_antievasion.sh",
        "auth11_required_suite_obligation_guards_contract",
        "exit status was `0`",
    ] {
        assert_contains(&gate_results, token);
    }

    let disposition = read(&format!("{PACKAGE}/artifacts/disposition.md"));
    assert_contains(&disposition, "Final disposition: `executed-hold`");
    assert_contains(&disposition, "fixed-baseline `stmtim` observe recovery");
    assert_contains(&disposition, "production_physics_edit_authorized: `false`");

    let review_disposition = read(&format!("{PACKAGE}/artifacts/review-disposition.md"));
    assert_contains(&review_disposition, "A-001");
    assert_contains(&review_disposition, "`accepted`");
    assert_contains(&review_disposition, "B-001");
    assert_contains(&review_disposition, "No undispositioned findings remain");

    let verification_a = read(&format!("{PACKAGE}/artifacts/verification_agent_a.md"));
    assert_contains(&verification_a, "Final verification: PASS");
    let verification_b = read(&format!("{PACKAGE}/artifacts/verification_agent_b.md"));
    assert_contains(&verification_b, "Final verification: PASS");

    let handoff = read(&format!("{PACKAGE}/artifacts/worker-handoff.md"));
    assert_contains(&handoff, "Status: complete");
    assert_contains(&handoff, "Next package: `HPHYS0320`");
}
