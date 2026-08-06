use std::fs;

use serde_json::Value;

const CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const PACKAGE: &str =
    "docs/work-packages/20260627-snowdensity-10-3-13-residual-policy-b-diagnostic-001/package.md";
const TOOL: &str = "tools/snowfreeze_observed/residual_policy_b_diagnostic.py";
const REPORT: &str = concat!(
    "docs/work-packages/20260627-snowdensity-10-3-13-residual-policy-b-diagnostic-001/",
    "artifacts/residual-policy-b-diagnostic.json"
);

#[test]
fn contract_and_package_bind_residual_policy_b_diagnostic() {
    let contract = read(CONTRACT);
    for marker in [
        "contract_version: 127",
        "INV-SNOWFREEZE-070",
        "OBL-SNOWFREEZE-P-045",
        "SNOWDENSITY-10.3.13 Residual-Tail And Policy-B Diagnostic Addendum",
        "date-level residual transition report",
        "Policy-B workspace-suite/conservation evidence matrix",
        "This diagnostic does not amend `INV-SNOWFREEZE-003`",
    ] {
        assert_contains(&contract, marker, CONTRACT);
    }

    let package = read(PACKAGE);
    for marker in [
        "SNOWDENSITY-10.3.13 Residual Tail And Policy-B Diagnostic",
        "coe_liquid_holding_capacity_v1 + physics_bulk_density_compaction_v1",
        "Policy-B full-model-surface no-regression evidence",
        "Density-cap changes, including `550 kg m^-3` SNOBAL cap re-anchoring",
        "HOLD-ACTIVATION-EVIDENCE-MISSING",
    ] {
        assert_contains(&package, marker, PACKAGE);
    }
}

#[test]
fn tool_preserves_diagnostic_boundaries_and_uses_real_wat_lineage() {
    let tool = read(TOOL);
    for marker in [
        "bundle-activation-adjudication.json",
        "load_modeled_wat",
        "prior_holding_capacity_only",
        "prior_spring_densification",
        "POLICY-B-FULL-SURFACE-NO-REGRESSION-EVIDENCE-MISSING",
        "FOLLOW_UP_ONLY_NOT_EVALUATED_HERE",
        "density_cap_changed",
        "qwet_or_frzftp_changed",
    ] {
        assert_contains(&tool, marker, TOOL);
    }
}

#[test]
fn executed_report_records_transition_diagnostic_not_activation() {
    let report: Value =
        serde_json::from_str(&read(REPORT)).expect("residual diagnostic report should parse");
    assert_eq!(
        report["schema"],
        "snowdensity10-3-13-residual-policy-b-diagnostic-v1"
    );
    assert_eq!(
        report["summary"]["disposition"],
        "HOLD-ACTIVATION-EVIDENCE-MISSING"
    );
    assert_eq!(report["summary"]["activation_ready"], false);
    assert_eq!(
        report["summary"]["activation_blocker"],
        "POLICY-B-FULL-SURFACE-NO-REGRESSION-EVIDENCE-MISSING"
    );
    assert_eq!(
        report["summary"]["frost_attribution_blocker"],
        "SNOW-CONTROL-RESIDUALS-REMAIN"
    );
    assert_eq!(
        report["protected_boundaries"]["default_activation_changed"],
        false
    );
    assert_eq!(report["protected_boundaries"]["density_cap_changed"], false);
    assert_eq!(
        report["active_density_cap"]["snobal_550_reanchor_status"],
        "FOLLOW_UP_ONLY_NOT_EVALUATED_HERE"
    );
    assert!(
        report["summary"]["under_persistence_induced_by_bundle_density_arm_count"]
            .as_u64()
            .expect("induced under-persistence count")
            > 0,
        "diagnostic should expose the density-arm mechanism-cost signal"
    );
    assert!(
        report["policy_b_evidence_matrix"]
            .as_array()
            .expect("policy matrix")
            .iter()
            .any(|row| row["status"] == "MISSING"),
        "Policy-B activation evidence must remain incomplete in this diagnostic"
    );
}

fn read(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|err| panic!("failed to read {path}: {err}"))
}

fn assert_contains(text: &str, marker: &str, path: &str) {
    assert!(
        text.contains(marker),
        "expected {path} to contain marker: {marker}"
    );
}
