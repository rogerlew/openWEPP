use std::fs;

use serde_json::Value;

const CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const PACKAGE: &str =
    "docs/work-packages/20260627-snowdensity-10-3-12-bundle-activation-adjudication-001/package.md";
const TOOL: &str = "tools/snowfreeze_observed/bundle_activation_adjudication.py";
const REPORT: &str = concat!(
    "docs/work-packages/20260627-snowdensity-10-3-12-bundle-activation-adjudication-001/",
    "artifacts/bundle-activation-adjudication.json"
);

#[test]
fn contract_and_package_bind_bundle_activation_boundary() {
    let contract = read(CONTRACT);
    for marker in [
        "contract_version: 112",
        "INV-SNOWFREEZE-069",
        "OBL-SNOWFREEZE-P-044",
        "SNOWDENSITY-10.3.12 Combined Bundle Activation Adjudication Addendum",
        "Activation Policy B supersedes any zero-paired-snow-failure activation rule",
        "workspace-suite no-regression gate",
        "Observation-blocked surfaces remain diagnostic-only",
    ] {
        assert_contains(&contract, marker, CONTRACT);
    }

    let package = read(PACKAGE);
    for marker in [
        "SNOWDENSITY-10.3.12 Bundle Activation Adjudication",
        "coe_liquid_holding_capacity_v1 + physics_bulk_density_compaction_v1",
        "`ACTIVATION-READY` only if Policy B is satisfied",
        "HOLD-OPT-IN-BUNDLE",
        "RETIRE-BUNDLE",
        "Frost attribution remains blocked",
    ] {
        assert_contains(&package, marker, PACKAGE);
    }
}

#[test]
fn tool_uses_real_direct_bundle_and_preserves_boundaries() {
    let tool = read(TOOL);
    for marker in [
        "OPENWEPP_SNOWDENSITY1038_MELT_MODEL",
        "OPENWEPP_SNOWDENSITY09_DENSITY_MODEL",
        "OPENWEPP_R7H_SNOW_TRACE_PATH",
        "coe_liquid_holding_capacity_v1",
        "physics_bulk_density_compaction_v1",
        "real direct-production WAT via openwepp-cli-hill",
        "default_activation_changed",
        "public_output_schema_changed",
        "new_process_physics_added",
        "qwet_or_frzftp_changed",
    ] {
        assert_contains(&tool, marker, TOOL);
    }
    assert_contains(&tool, "subprocess.run", TOOL);
}

#[test]
fn executed_report_records_hold_not_activation() {
    let report: Value =
        serde_json::from_str(&read(REPORT)).expect("bundle activation report should be valid JSON");
    assert_eq!(
        report["schema"],
        "snowdensity10-3-12-bundle-activation-adjudication-v1"
    );
    assert_eq!(
        report["bundle"]["snow_melt_model"],
        "coe_liquid_holding_capacity_v1"
    );
    assert_eq!(
        report["bundle"]["snow_density_model"],
        "physics_bulk_density_compaction_v1"
    );
    assert_eq!(
        report["protected_boundaries"]["default_activation_changed"],
        false
    );
    assert_eq!(
        report["protected_boundaries"]["new_process_physics_added"],
        false
    );
    assert_eq!(report["summary"]["activation_ready"], false);
    assert_eq!(report["summary"]["activation_policy"], "POLICY-B");
    assert_eq!(
        report["summary"]["blocker"],
        "POLICY-B-FULL-SURFACE-NO-REGRESSION-EVIDENCE-MISSING"
    );
    assert_eq!(
        report["summary"]["policy_b_zero_paired_snow_failures_required"],
        false
    );
    assert_eq!(
        report["summary"]["policy_b_gate_eligible_snow_strictly_better_than_default"],
        true
    );
    assert_eq!(
        report["summary"]["policy_b_full_surface_no_regression_evidence_present"],
        false
    );
    assert_eq!(report["summary"]["frost_attribution_unblocked"], false);
    assert!(
        report["summary"]["bundle_snow_control_fail_count"]
            .as_u64()
            .expect("bundle failure count")
            > 0,
        "frost attribution must remain blocked while snow-control residuals remain"
    );
    assert!(
        report["bundle"]["trace_proof"]["bundle_snow_melt_model_count"]
            .as_u64()
            .expect("melt trace count")
            > 0
    );
    assert!(
        report["bundle"]["trace_proof"]["bundle_snow_density_model_count"]
            .as_u64()
            .expect("density trace count")
            > 0
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
