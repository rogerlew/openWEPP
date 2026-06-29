use std::fs;

use serde_json::Value;

const CONTRACT: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const PACKAGE: &str = concat!(
    "docs/work-packages/20260627-snowdensity-10-3-14-policy-b-no-regression-cap-authority-001/",
    "package.md"
);
const TOOL: &str = "tools/snowfreeze_observed/policy_b_no_regression_cap_authority.py";
const REPORT: &str = concat!(
    "docs/work-packages/20260627-snowdensity-10-3-14-policy-b-no-regression-cap-authority-001/",
    "artifacts/policy-b-no-regression-cap-authority.json"
);

#[test]
fn contract_and_package_bind_policy_b_cap_authority() {
    let contract = read(CONTRACT);
    for marker in [
        "contract_version: 108",
        "INV-SNOWFREEZE-071",
        "OBL-SNOWFREEZE-P-046",
        "SNOWDENSITY-10.3.14 Policy-B No-Regression And Cap Authority Addendum",
        "workspace-suite no-regression gate",
        "same-SWE, cap-pinned projection",
        "`550 kg m^-3` remains projection-only",
    ] {
        assert_contains(&contract, marker, CONTRACT);
    }

    let package = read(PACKAGE);
    for marker in [
        "SNOWDENSITY-10.3.14 Policy-B No-Regression And Cap Authority",
        "Run the workspace no-regression gate under the existing package-bound opt-in",
        "Changing `INV-SNOWFREEZE-003` density cap",
        "No production cap/default/schema/fixture/runtime-selector changes are made",
    ] {
        assert_contains(&package, marker, PACKAGE);
    }
}

#[test]
fn tool_preserves_active_cap_and_protected_boundaries() {
    let tool = read(TOOL);
    for marker in [
        "snowdensity10-3-14-policy-b-no-regression-cap-authority-v1",
        "INV-SNOWFREEZE-071 OBL-SNOWFREEZE-P-046",
        "ACTIVE_CAP_KG_M3 = 522.0",
        "PROJECTED_CAP_KG_M3 = 550.0",
        "workspace_regression_status",
        "default_activation_changed",
        "density_cap_changed",
        "projection_type",
        "static same-SWE cap-pinned depth projection; not a dynamic rerun",
        "cap_reanchor_required_for_activation",
    ] {
        assert_contains(&tool, marker, TOOL);
    }
}

#[test]
fn executed_report_records_policy_b_readiness_under_active_cap_only() {
    let report: Value = serde_json::from_str(&read(REPORT)).expect("10.3.14 report should parse");
    assert_eq!(
        report["schema"],
        "snowdensity10-3-14-policy-b-no-regression-cap-authority-v1"
    );
    assert_eq!(
        report["active_density_cap"]["changed_by_this_package"],
        false
    );
    assert_eq!(
        report["projected_density_cap"]["changed_by_this_package"],
        false
    );
    assert_eq!(
        report["protected_boundaries"]["default_activation_changed"],
        false
    );
    assert_eq!(report["protected_boundaries"]["density_cap_changed"], false);
    assert_eq!(
        report["summary"]["activation_package_ready_under_active_cap"],
        true
    );
    assert_eq!(
        report["summary"]["cap_reanchor_required_for_activation"],
        false
    );
    assert_eq!(
        report["summary"]["density_cap_changed"], false,
        "diagnostic must not mutate the runtime cap"
    );
    assert!(
        report["summary"]["cap_pinned_paired_row_count"]
            .as_u64()
            .expect("cap-pinned count")
            > 0,
        "cap authority diagnostic should evaluate cap-pinned rows"
    );
    assert!(
        report["summary"]["cap_pinned_pass_to_fail_count"]
            .as_u64()
            .expect("pass-to-fail count")
            > 0,
        "550 kg m^-3 projection must expose its under-persistence risk"
    );
    assert_eq!(
        report["summary"]["cap_reanchor_disposition"],
        "MIXED-FOLLOW-UP-DYNAMIC-RERUN-REQUIRED"
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
