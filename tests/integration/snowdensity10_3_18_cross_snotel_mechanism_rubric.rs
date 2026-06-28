use std::fs;

const TOOL: &str = "tools/snowfreeze_observed/cross_snotel_mechanism_rubric.py";
const PACKAGE: &str =
    "docs/work-packages/20260627-snowdensity-10-3-18-cross-snotel-mechanism-rubric-001/package.md";

#[test]
fn cross_snotel_rubric_tool_is_diagnostic_and_contract_bound() {
    let tool = fs::read_to_string(TOOL).expect("read 10.3.18 diagnostic tool");

    for required in [
        "INV-SNOWFREEZE-050",
        "REF-SNOWFREEZE-FROST-OBS",
        "ADR-0017",
        "diagnostic_only",
        "no_promotion_or_activation_decision",
        "legacy_and_pysnobal_are_flags_not_targets",
        "absolute_swe_depth_cells_are_report_only",
    ] {
        assert!(tool.contains(required), "{TOOL} missing {required}");
    }

    for model in [
        "legacy_baseline",
        "activated_bundle",
        "harder_pomeroy_partition",
        "open_sublimation_stage_a_10_3_16",
        "shallow_pack_guard_10_3_17",
        "spring_densification_10_3_11",
        "winter_thaw_state_loss_10_3_7",
        "pysnobal_reference",
    ] {
        assert!(tool.contains(model), "{TOOL} missing model {model}");
    }

    assert!(tool.contains("physics_bulk_shallow_guard_v1"));
    assert!(tool.contains("10.3.17 non-promoted opt-in profile folded into the model list"));
    assert!(tool.contains("archival_not_current_selector"));
    assert!(tool.contains("archival_snowbench_only"));
}

#[test]
fn package_keeps_no_activation_boundary() {
    let package = fs::read_to_string(PACKAGE).expect("read 10.3.18 package");

    for required in [
        "diagnostic-only",
        "NO promotion/activation decision",
        "No production/default/cap/schema/fixture/frost change",
        "legacy/PySnobal are flags, not targets",
        "INV-SNOWFREEZE-050",
    ] {
        assert!(package.contains(required), "{PACKAGE} missing {required}");
    }
}
