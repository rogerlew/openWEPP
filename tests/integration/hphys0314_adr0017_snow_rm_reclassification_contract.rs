use std::fs;
use std::path::Path;

const PACKAGE: &str =
    "docs/work-packages/20260606-hphys0314-adr0017-snow-rm-reclassification-route-ledger-001";

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("failed to read {path}: {error}"))
}

#[test]
fn hphys0314_contract_authority_is_registered() {
    let snowfreeze = read("docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md");
    assert!(snowfreeze.contains("contract_version:"));
    assert!(snowfreeze.contains("INV-SNOWFREEZE-040"));
    assert!(
        snowfreeze.contains("HPHYS0314 ADR0017 snow/`RM` route-ledger reclassification invariant")
    );
    assert!(snowfreeze.contains("OBL-SNOWFREEZE-P-019"));
    assert!(snowfreeze.contains("SC-WATBAL-001#INV-WATBAL-088"));
    assert!(snowfreeze.contains("hourly-snowfall-input-lineage-hold"));
    assert!(snowfreeze.contains("recursive-year-start-inherited-state-hold"));

    let watbal = read("docs/specifications/science-contracts/contracts/SC-WATBAL-001.md");
    assert!(watbal.contains("contract_version:"));
    assert!(watbal.contains("INV-WATBAL-088"));
    assert!(watbal.contains(
        "HPHYS0314 ADR0017 snow/`RM` water-balance route-ledger reclassification invariant"
    ));
    assert!(watbal.contains("OBL-WATBAL-P-024"));
    assert!(watbal.contains("SC-SNOWFREEZE-001#INV-SNOWFREEZE-040"));

    let index = read("docs/specifications/science-contracts/index.md");
    assert!(index.contains("SC-SNOWFREEZE-001#INV-SNOWFREEZE-040"));
    assert!(index.contains("SC-WATBAL-001#INV-WATBAL-088"));
}

#[test]
fn hphys0314_package_is_autonomous_no_production_edits() {
    let package = read(&format!("{PACKAGE}/package.md"));
    assert!(package.contains("Status:"));
    assert!(package.contains("Contract-First Sequence"));
    assert!(package.contains("Record full H1..H39 continuation metrics"));
    assert!(package.contains("static carry-forward"));
    assert!(package.contains("No production Rust kernel edits"));
    assert!(package.contains("dual review"));
    assert!(package.contains("dual verification"));
    assert!(package.contains("docs/work-packages/20260605-hphys0313-snowpack-settling-carry-recursion-closure-001/artifacts/snowpack-settling-carry-recursion-ledger.json"));

    let prompt = read(&format!(
        "{PACKAGE}/prompts/active/20260606-hphys0314-adr0017-snow-rm-reclassification-route-ledger-001_kickoff_agent_prompt.md"
    ));
    assert!(prompt.contains("Execution mode: package-end-to-end"));
    assert!(prompt.contains("Required reading"));
    assert!(prompt.contains("Autonomy:"));
    assert!(prompt.contains("no external connectivity"));
}

#[test]
fn hphys0314_reclassification_ledger_preserves_routes_and_gates_followons() {
    let ledger_path =
        format!("{PACKAGE}/artifacts/adr0017-snow-rm-reclassification-route-ledger.md");
    assert!(
        Path::new(&ledger_path).exists(),
        "required HPHYS0314 route ledger artifact is missing"
    );
    let ledger = read(&ledger_path);
    assert!(!ledger.contains("Status: queued"));
    assert!(ledger.contains("Evidence mode: Static"));
    assert!(ledger.contains("ADR0017 verdict"));
    assert!(ledger.contains("HARNESS-SURFACE-MISMATCH"));
    assert!(ledger.contains("LEGACY-DEFECTIVE"));
    assert!(ledger.contains("OPENWEPP-DEFECTIVE"));
    assert!(ledger.contains("UNRESOLVED"));
    assert!(ledger.contains("hourly-snowfall-input-lineage-hold"));
    assert!(ledger.contains("recursive-year-start-inherited-state-hold"));
    assert!(ledger.contains("HPHYS0315"));
    assert!(ledger.contains("HPHYS0316"));
    assert!(ledger.contains("production_edit_authorized=false"));
    assert!(ledger.contains("stale HPHYS0298 `OPENWEPP-DEFECTIVE`"));
    for token in [
        "| 1 | spring-2014 |",
        "| 1 | spring-2016 |",
        "| 7 | spring-2014 |",
        "| 7 | spring-2016 |",
        "| 39 | spring-2014 |",
        "| 39 | spring-2016 |",
    ] {
        assert!(ledger.contains(token), "ledger missing row token {token}");
    }
}

#[test]
fn hphys0314_route_summary_accounts_for_all_carried_rows() {
    let summary = read(&format!(
        "{PACKAGE}/artifacts/route-consolidation-summary.md"
    ));
    assert!(!summary.contains("Status: queued"));
    assert!(summary.contains("Evidence mode: Static"));
    assert!(summary.contains("Total carried rows: `57`"));
    assert!(summary.contains("hourly-snowfall-input-lineage-hold` | `3` | `24`"));
    assert!(summary.contains("recursive-year-start-inherited-state-hold` | `3` | `33`"));
    assert!(summary.contains("Production edits authorized: `0`"));
    assert!(summary.contains("HPHYS0315"));
    assert!(summary.contains("HPHYS0316"));
}

#[test]
fn hphys0314_metrics_and_evidence_artifacts_are_not_scaffold_placeholders() {
    for relative in [
        "artifacts/contract-implementation-evidence.md",
        "artifacts/contract-test-implementation-evidence.md",
        "artifacts/pre-implementation-contract-gate.md",
        "artifacts/implementation-test-evidence.md",
        "artifacts/kernel-profile-compliance-checklist.md",
        "artifacts/owned-file-manifest.md",
        "artifacts/gate-results.md",
        "artifacts/README.md",
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
    assert!(metrics.contains("H1..H39"));
    assert!(metrics.contains("Snow-Water"));
    assert!(metrics.contains("RM"));
    assert!(metrics.contains("0/39"));
    assert!(metrics.contains("carried forward"));

    let gate_results = read(&format!("{PACKAGE}/artifacts/gate-results.md"));
    assert!(gate_results.contains("cargo clippy --workspace --all-targets -- -D warnings"));
    assert!(gate_results.contains("cargo test --workspace"));
    assert!(gate_results.contains("cargo deny check"));
    assert!(gate_results.contains("exit status was `0`"));

    let disposition = read(&format!("{PACKAGE}/artifacts/disposition.md"));
    assert!(disposition.contains("Status: executed-hold"));
    assert!(disposition.contains("Final disposition: `executed-hold`"));
    assert!(!disposition.contains("Final disposition remains pending"));

    let verification_b = read(&format!("{PACKAGE}/artifacts/verification_agent_b.md"));
    assert!(verification_b.contains("Status: complete"));
    assert!(verification_b.contains("Final verification: PASS"));
    assert!(!verification_b.contains("Final verification: pending"));

    let handoff = read(&format!("{PACKAGE}/artifacts/worker-handoff.md"));
    assert!(handoff.contains("Status: complete"));
    assert!(handoff.contains("None for HPHYS0314"));
}
