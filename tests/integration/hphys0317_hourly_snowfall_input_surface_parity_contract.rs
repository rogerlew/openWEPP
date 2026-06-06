use std::fs;
use std::path::Path;

const PACKAGE: &str =
    "docs/work-packages/20260606-hphys0317-paired-hourly-snowfall-input-surface-parity-001";

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("failed to read {path}: {error}"))
}

fn assert_contains(content: &str, token: &str) {
    assert!(content.contains(token), "missing required token: {token}");
}

#[test]
fn hphys0317_contract_authority_is_registered() {
    let climate = read("docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md");
    assert_contains(&climate, "contract_version:");
    assert_contains(&climate, "INV-CLIMATE-015");
    assert_contains(
        &climate,
        "HPHYS0317 paired hourly snowfall input-surface parity invariant",
    );
    assert_contains(&climate, "OBL-CLIMATE-P-010");
    assert_contains(&climate, "SC-SNOWFREEZE-001#INV-SNOWFREEZE-043");
    assert_contains(&climate, "SC-WATBAL-001#INV-WATBAL-091");

    let snowfreeze = read("docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md");
    assert_contains(&snowfreeze, "contract_version:");
    assert_contains(&snowfreeze, "INV-SNOWFREEZE-043");
    assert_contains(
        &snowfreeze,
        "HPHYS0317 paired hourly snowfall input-surface closure invariant",
    );
    assert_contains(&snowfreeze, "OBL-SNOWFREEZE-P-022");
    assert_contains(&snowfreeze, "SC-CLIMATE-001#INV-CLIMATE-015");
    assert_contains(&snowfreeze, "SC-WATBAL-001#INV-WATBAL-091");

    let watbal = read("docs/specifications/science-contracts/contracts/SC-WATBAL-001.md");
    assert_contains(&watbal, "contract_version:");
    assert_contains(&watbal, "INV-WATBAL-091");
    assert_contains(
        &watbal,
        "HPHYS0317 paired hourly snowfall input-surface water-balance gate",
    );
    assert_contains(&watbal, "OBL-WATBAL-P-027");
    assert_contains(&watbal, "SC-SNOWFREEZE-001#INV-SNOWFREEZE-043");
    assert_contains(&watbal, "SC-CLIMATE-001#INV-CLIMATE-015");

    let index = read("docs/specifications/science-contracts/index.md");
    assert_contains(&index, "SC-CLIMATE-001#INV-CLIMATE-015");
    assert_contains(&index, "SC-SNOWFREEZE-001#INV-SNOWFREEZE-043");
    assert_contains(&index, "SC-WATBAL-001#INV-WATBAL-091");
}

#[test]
fn hphys0317_package_is_autonomous_and_scoped_to_no_production_edits() {
    let package = read(&format!("{PACKAGE}/package.md"));
    assert_contains(&package, "Status: executed-hold");
    assert_contains(&package, "Contract-First Sequence");
    assert_contains(&package, "No production Rust kernel edits");
    assert_contains(&package, "paired hourly snowfall input-surface parity");
    assert_contains(&package, "HPHYS0318");
    assert_contains(&package, "No WB13, WB17, WB18, WB19, WB12");
    assert_contains(&package, "Dual review findings");

    let prompt = read(&format!(
        "{PACKAGE}/prompts/active/20260606-hphys0317-paired-hourly-snowfall-input-surface-parity-001_kickoff_agent_prompt.md"
    ));
    assert_contains(&prompt, "Execution mode: package-end-to-end");
    assert_contains(&prompt, "Required reading");
    assert_contains(&prompt, "Autonomy:");
    assert_contains(&prompt, "no external connectivity");
}

#[test]
fn hphys0317_ledger_preserves_all_carried_rows_under_instrumentation_hold() {
    let ledger_path = format!("{PACKAGE}/artifacts/paired-input-surface-ledger.md");
    assert!(
        Path::new(&ledger_path).exists(),
        "required HPHYS0317 paired input-surface ledger artifact is missing"
    );
    let ledger = read(&ledger_path);
    assert!(!ledger.contains("Status: queued"));
    assert!(!ledger.contains("Evidence mode: not-run"));
    assert_contains(&ledger, "Evidence mode: Static");
    assert_contains(&ledger, "Total carried rows: `57`");
    assert_contains(&ledger, "Spring-2014 carried rows: `24`");
    assert_contains(&ledger, "Spring-2016 carried rows: `33`");
    assert_contains(&ledger, "owner: `HPHYS0318`");
    assert_contains(&ledger, "ADR0017 verdict: `UNRESOLVED`");
    assert_contains(&ledger, "paired-input-surface-instrumentation-hold");
    assert_contains(&ledger, "production_edit_authorized=false");
    assert_contains(&ledger, "`2013 day 11 hour 11`");
    assert_contains(&ledger, "`hrsnow = 0.0007454545120708644 m`");
    assert_contains(&ledger, "`snow.hourly.snowfall_m_0011 = 0.0 m`");
    for token in [
        "| H1 | spring-2014 | 8 |",
        "| H7 | spring-2014 | 7 |",
        "| H39 | spring-2014 | 9 |",
        "| H1 | spring-2016 | 15 |",
        "| H7 | spring-2016 | 9 |",
        "| H39 | spring-2016 | 9 |",
    ] {
        assert_contains(&ledger, token);
    }
}

#[test]
fn hphys0317_source_lineage_records_missing_controlling_surface_evidence() {
    let lineage = read(&format!(
        "{PACKAGE}/artifacts/paired-input-surface-source-lineage.md"
    ));
    assert!(!lineage.contains("Status: queued"));
    assert_contains(&lineage, "Evidence mode: Static");
    for token in [
        "`winter.for:296-300`",
        "`stmtim.for:35-38`",
        "`stmtim.for:43-95`",
        "`stmtim.for:77-95`",
        "`snowd.for:166-172`",
        "`06_simimpl28_hourly_forcing.rs:57-71`",
        "`06_simimpl28_hourly_forcing.rs:627-697`",
        "`rain`",
        "`stmdur`",
        "`wntdur`",
        "`wnttim`",
        "`hrtemp`",
        "`rst`",
        "`hrsnow`",
        "`hrrain`",
    ] {
        assert_contains(&lineage, token);
    }
    assert_contains(&lineage, "source-code resemblance is not parity proof");
    assert_contains(&lineage, "no production edit is authorized");
}

#[test]
fn hphys0317_artifacts_are_executed_and_gates_are_recorded() {
    for relative in [
        "artifacts/README.md",
        "artifacts/contract-implementation-evidence.md",
        "artifacts/contract-test-implementation-evidence.md",
        "artifacts/pre-implementation-contract-gate.md",
        "artifacts/implementation-test-evidence.md",
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
        "artifacts/full-39-suite-metrics.md",
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
    assert_contains(&metrics, "carried forward");
    assert_contains(&metrics, "no production runtime code changed");

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
    assert_contains(&disposition, "HPHYS0318");
    assert_contains(&disposition, "No production code edits were made");

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
    assert_contains(&handoff, "Next package: `HPHYS0318`");
}
