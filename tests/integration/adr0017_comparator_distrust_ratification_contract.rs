use std::{fs, path::Path};

const PACKAGE: &str = "docs/work-packages/20260605-adr0017-comparator-distrust-ratification-001";
const ADR0016: &str = "docs/decisions/0016-promote-260430-baseline-as-canonical-comparator-and-abandon-kernel-rewrite.md";
const ADR0017: &str =
    "docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md";

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("failed to read {path}: {error}"))
}

#[test]
fn adr0017_is_accepted_and_registered_as_ratified_governance() {
    let adr0017 = read(ADR0017);
    let adr0016 = read(ADR0016);
    let decisions = read("docs/decisions/README.md");

    assert!(adr0017.contains("**Status:** Accepted"));
    assert!(!adr0017.contains("**Status:** Proposed"));
    assert!(adr0017.contains(
        "ratified by Codex package `20260605-adr0017-comparator-distrust-ratification-001`"
    ));
    assert!(adr0017.contains("wepp_260430_negmeltfix_comparator"));
    assert!(adr0017.contains("wepp_260430_negmeltfix_comparator_47ac4c32faee"));
    assert!(adr0017.contains("47ac4c32faeea81bb99081f955a14c38b815ef4d"));
    assert!(adr0017.contains("HARNESS-SURFACE-MISMATCH"));
    assert!(adr0017.contains("Criterion C is not waivable"));

    assert!(adr0016.contains(
        "**Amended By:** [ADR-0017](0017-re-pin-operational-distrust-comparator-is-flag-not-target.md) (accepted)"
    ));
    assert!(decisions.contains(
        "| [0017](0017-re-pin-operational-distrust-comparator-is-flag-not-target.md) | Re-pin operational distrust — the fixed comparator is a flag, not a target | Accepted"
    ));
    assert!(
        Path::new(PACKAGE).exists(),
        "ADR-0017 ratification package is not registered at expected path: {PACKAGE}"
    );
}

#[test]
fn governance_docs_encode_comparator_flag_adjudication_gates() {
    let correctness = read("docs/specifications/correctness-authority-model.md");
    assert!(correctness.contains("## ADR-0017 Comparator-Flag Adjudication (Normative)"));
    for verdict in [
        "HARNESS-SURFACE-MISMATCH",
        "LEGACY-DEFECTIVE",
        "OPENWEPP-DEFECTIVE",
        "UNRESOLVED",
    ] {
        assert!(correctness.contains(verdict), "missing verdict {verdict}");
    }
    assert!(
        correctness
            .contains("Criterion-C-style independent correctness authority may not be waived")
    );
    assert!(correctness.contains("approximately `10x` or `1000x`"));
    assert!(correctness.contains("Ownerless or unscoped `HOLD` findings are"));

    let procedure = read("docs/specifications/science-contract-authoring-procedure.md");
    assert!(procedure.contains("ADR-0017 comparator-distrust governance"));
    assert!(procedure.contains("`HARNESS-SURFACE-MISMATCH` verdict"));
    assert!(procedure.contains("owner/follow-on package"));
    assert!(procedure.contains("approximately `10x` for snow depth"));
    assert!(procedure.contains("`1000x` for meters versus millimetres"));

    let plans = read("docs/codex_exec_plans.md");
    assert!(plans.contains("ADR-0017"));
    assert!(plans.contains("HARNESS-SURFACE-MISMATCH"));
    assert!(plans.contains("must prohibit waiving independent"));
    assert!(plans.contains("correctness authority for openWEPP-defect labels"));

    let units = read("docs/specifications/unit-governance.md");
    assert!(units.contains("Depth versus"));
    assert!(units.contains("snow-water-equivalent"));
    assert!(units.contains("unit and lineage"));
    assert!(units.contains("stage before a residual"));
    assert!(units.contains("OPENWEPP-DEFECTIVE"));

    let index = read("docs/specifications/science-contracts/index.md");
    assert!(index.contains("ADR0017 registry note"));
    assert!(index.contains("comparator agreement is a flag, not a target"));
    assert!(index.contains("SC-WATBAL-001#INV-WATBAL-087"));
}

#[test]
fn sc_contracts_carry_adr0017_invariants_obligations_and_versions() {
    let snowfreeze = read("docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md");
    assert!(snowfreeze.contains("contract_version:"));
    assert!(snowfreeze.contains("INV-SNOWFREEZE-039"));
    assert!(snowfreeze.contains("ADR0017 snow/`RM` comparator verdict invariant"));
    assert!(snowfreeze.contains("SC-WATBAL-001#INV-WATBAL-087"));
    assert!(snowfreeze.contains("OBL-SNOWFREEZE-P-018"));
    assert!(snowfreeze.contains("HARNESS-SURFACE-MISMATCH"));
    assert!(snowfreeze.contains("Criterion C may not be waived"));

    let watbal = read("docs/specifications/science-contracts/contracts/SC-WATBAL-001.md");
    assert!(watbal.contains("contract_version:"));
    assert!(watbal.contains("INV-WATBAL-087"));
    assert!(watbal.contains("ADR0017 water-balance comparator verdict invariant"));
    assert!(watbal.contains("SC-SNOWFREEZE-001#INV-SNOWFREEZE-039"));
    assert!(watbal.contains("ADR0017 Comparator-Flag Water-Balance Addendum"));
    assert!(watbal.contains("OBL-WATBAL-P-023"));
    assert!(watbal.contains("HARNESS-SURFACE-MISMATCH"));
    assert!(watbal.contains("all `HOLD` dispositions must name a scoped owner"));
}

#[test]
fn ratification_package_is_autonomous_and_truthfulness_labeled() {
    let package = read(&format!("{PACKAGE}/package.md"));
    assert!(package.contains("Execution mode: package-end-to-end"));
    assert!(package.contains("Autonomy"));
    assert!(package.contains("Contract-first sequence"));
    assert!(package.contains("dual review"));
    assert!(package.contains("dual verification"));
    assert!(package.contains("No production Rust kernel edits"));
    assert!(package.contains("truthfulness labeling"));

    let prompt = read(&format!(
        "{PACKAGE}/prompts/active/20260605-adr0017-comparator-distrust-ratification-001_kickoff_agent_prompt.md"
    ));
    assert!(prompt.contains("Scope: local repository governance/contract ratification task"));
    assert!(prompt.contains("Execution mode: package-end-to-end"));
    assert!(prompt.contains("Required reading"));
    assert!(prompt.contains("Autonomy: execute package phases end-to-end"));
    assert!(prompt.contains("no external connectivity"));

    for artifact in [
        "contract-implementation-evidence.md",
        "contract-test-implementation-evidence.md",
        "pre-implementation-contract-gate.md",
        "implementation-test-evidence.md",
        "kernel-profile-compliance-checklist.md",
        "owned-file-manifest.md",
        "gate-results.md",
        "disposition.md",
        "worker-handoff.md",
        "review_agent_a.md",
        "review_agent_b.md",
        "verification_agent_a.md",
        "verification_agent_b.md",
        "review-disposition.md",
    ] {
        let contents = read(&format!("{PACKAGE}/artifacts/{artifact}"));
        assert!(
            contents.contains("Status:"),
            "artifact {artifact} must carry a status"
        );
        assert!(
            contents.contains("Evidence mode:"),
            "artifact {artifact} must carry evidence mode"
        );
        let status_line = contents
            .lines()
            .find(|line| line.starts_with("Status:"))
            .unwrap_or_else(|| panic!("artifact {artifact} missing status line"));
        let evidence_line = contents
            .lines()
            .find(|line| line.starts_with("Evidence mode:"))
            .unwrap_or_else(|| panic!("artifact {artifact} missing evidence mode line"));
        assert!(
            status_line != "Status: queued",
            "artifact {artifact} must not remain queued"
        );
        assert!(
            evidence_line != "Evidence mode: not-run",
            "artifact {artifact} must not remain not-run"
        );
        assert!(
            status_line != "Status: in_progress",
            "artifact {artifact} must not remain in_progress at closeout"
        );
    }

    let gate_results = read(&format!("{PACKAGE}/artifacts/gate-results.md"));
    assert!(gate_results.contains("Ran:"));
    assert!(gate_results.contains("PASS"));

    let disposition = read(&format!("{PACKAGE}/artifacts/disposition.md"));
    assert!(disposition.contains("Status: complete"));
    assert!(disposition.contains("Disposition: accepted"));

    for artifact in [
        "review_agent_a.md",
        "review_agent_b.md",
        "verification_agent_a.md",
        "verification_agent_b.md",
    ] {
        let contents = read(&format!("{PACKAGE}/artifacts/{artifact}"));
        assert!(
            contents.contains("Verdict:"),
            "artifact {artifact} must carry a verdict"
        );
    }
}
