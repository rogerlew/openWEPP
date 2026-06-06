use std::fs;
use std::path::Path;

const PACKAGE: &str =
    "docs/work-packages/20260606-hphys0316-2013-terminal-carry-recursion-closure-001";

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("failed to read {path}: {error}"))
}

fn assert_contains(content: &str, token: &str) {
    assert!(content.contains(token), "missing required token: {token}");
}

#[test]
fn hphys0316_contract_authority_is_registered() {
    let snowfreeze = read("docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md");
    assert_contains(&snowfreeze, "contract_version: 47");
    assert_contains(&snowfreeze, "INV-SNOWFREEZE-042");
    assert_contains(
        &snowfreeze,
        "HPHYS0316 2013 terminal carry-recursion invariant",
    );
    assert_contains(&snowfreeze, "OBL-SNOWFREEZE-P-021");
    assert_contains(&snowfreeze, "SC-WATBAL-001#INV-WATBAL-090");
    assert_contains(&snowfreeze, "33` carried spring-2016 rows");
    assert_contains(&snowfreeze, "2013 day 11 hour 11");

    let watbal = read("docs/specifications/science-contracts/contracts/SC-WATBAL-001.md");
    assert_contains(&watbal, "contract_version: 140");
    assert_contains(&watbal, "INV-WATBAL-090");
    assert_contains(
        &watbal,
        "HPHYS0316 2013 terminal carry-recursion water-balance gate",
    );
    assert_contains(&watbal, "OBL-WATBAL-P-026");
    assert_contains(&watbal, "SC-SNOWFREEZE-001#INV-SNOWFREEZE-042");
    assert_contains(&watbal, "WB13/WB17/WB18/WB19/WB12");

    let index = read("docs/specifications/science-contracts/index.md");
    assert_contains(&index, "SC-SNOWFREEZE-001#INV-SNOWFREEZE-042");
    assert_contains(&index, "SC-WATBAL-001#INV-WATBAL-090");
}

#[test]
fn hphys0316_package_is_autonomous_and_scoped_to_no_production_edits() {
    let package = read(&format!("{PACKAGE}/package.md"));
    assert_contains(&package, "Status: executed-hold");
    assert_contains(&package, "Contract-First Sequence");
    assert_contains(&package, "No production Rust kernel edits");
    assert_contains(&package, "static carry-forward");
    assert_contains(&package, "HPHYS0317");
    assert_contains(&package, "No WB13, WB17, WB18, WB19, WB12");
    assert_contains(&package, "Dual review findings");

    let prompt = read(&format!(
        "{PACKAGE}/prompts/active/20260606-hphys0316-2013-terminal-carry-recursion-closure-001_kickoff_agent_prompt.md"
    ));
    assert_contains(&prompt, "Execution mode: package-end-to-end");
    assert_contains(&prompt, "Required reading");
    assert_contains(&prompt, "Autonomy:");
    assert_contains(&prompt, "no external connectivity");
}

#[test]
fn hphys0316_ledger_routes_all_spring_2016_rows_to_hphys0317_hold() {
    let ledger_path = format!("{PACKAGE}/artifacts/2013-terminal-carry-recursion-ledger.md");
    assert!(
        Path::new(&ledger_path).exists(),
        "required HPHYS0316 recursion ledger artifact is missing"
    );
    let ledger = read(&ledger_path);
    assert!(!ledger.contains("Status: queued"));
    assert!(!ledger.contains("Evidence mode: not-run"));
    assert_contains(&ledger, "Evidence mode: Static");
    assert_contains(&ledger, "Total carried rows: `33`");
    assert_contains(&ledger, "production_edit_authorized=false");
    assert_contains(&ledger, "owner: `HPHYS0317`");
    assert_contains(&ledger, "ADR0017 verdict: `UNRESOLVED`");
    assert_contains(&ledger, "2013-hourly-snowfall-input-surface-parity-hold");
    assert_contains(&ledger, "0.0007454545120708644");
    assert_contains(&ledger, "`snow.hourly.snowfall_m_0011 = 0.0 m`");
    assert_contains(&ledger, "`2013 day 11 hour 11`");
    assert_contains(&ledger, "`2014 day 1 hour 1`");
    for token in [
        "| H1 | spring-2016 | 15 | 0.013144251023522513 | 0.013144251023522124 |",
        "| H7 | spring-2016 | 9 | 0.015279465660242741 | 0.015279465660242408 |",
        "| H39 | spring-2016 | 9 | 0.0147979087518893 | 0.014797908751889022 |",
    ] {
        assert_contains(&ledger, token);
    }
}

#[test]
fn hphys0316_source_lineage_records_terminal_carry_and_first_material_lane() {
    let lineage = read(&format!(
        "{PACKAGE}/artifacts/2013-terminal-carry-source-lineage.md"
    ));
    assert!(!lineage.contains("Status: queued"));
    assert_contains(&lineage, "Evidence mode: Static");
    for token in [
        "`winter.for:193`",
        "`snowd.for:50-53`",
        "`snowd.for:61-65`",
        "`snowd.for:122-139`",
        "`snowd.for:166-172`",
        "`snowd.for:303-312`",
        "`03_kernel_support_00_support_helpers.rs:3872-3877`",
        "`03_kernel_support_00_support_helpers.rs:3901-3912`",
        "`03_kernel_support_00_support_helpers.rs:3914-3924`",
        "`HPHYS0312`",
        "`HPHYS0313`",
        "`HPHYS0315`",
        "`HPHYS0317`",
    ] {
        assert_contains(&lineage, token);
    }
    assert_contains(&lineage, "Remaining gap");
    assert_contains(&lineage, "no production edit is authorized");
}

#[test]
fn hphys0316_artifacts_are_executed_and_gates_are_recorded() {
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
    assert_contains(&disposition, "HPHYS0317");
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
    assert_contains(&handoff, "Next package: `HPHYS0317`");
}
