use std::fs::read_to_string;

const ORCHESTRATOR: &str = "crates/openwepp-hillslope-orchestrator/src";
const RUNNER: &str =
    "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers";

fn read(path: &str) -> String {
    read_to_string(path).unwrap_or_else(|error| panic!("{path}: {error}"))
}

#[test]
fn closure_path_uses_typed_attachment_and_keeps_legacy_handoff_test_only() {
    let runtime = read(&format!("{ORCHESTRATOR}/direct_runtime.rs"));
    let frames = read(&format!("{ORCHESTRATOR}/direct_runtime/00_core_frames.rs"));
    let scheduler = read(&format!(
        "{ORCHESTRATOR}/direct_runtime/snow_stage3_v11_scheduler.rs"
    ));
    let legacy = read(&format!(
        "{ORCHESTRATOR}/direct_runtime/snow_stage3_shadow.rs"
    ));

    assert!(runtime.contains("mod snow_stage3_shadow;"));
    assert!(frames.contains("pub snow_stage3_shadow:"));
    assert!(runtime.contains("mod snow_stage3_v11_scheduler;"));
    assert!(scheduler.contains("snow_stage3_v11_attachment"));
    assert!(scheduler.contains("sealed 48-support capability"));
    assert!(scheduler.contains("#[cfg(test)]\n        if let Some(mut attachment)"));
    assert!(!scheduler.contains("SnowStage3HandoffRuntime"));
    assert!(!scheduler.contains("TerminalStateRates"));
    assert!(legacy.contains("stage_after_live_day"));
    assert!(legacy.contains("restart_v1"));
}

#[test]
fn runner_persistent_state_is_explicitly_historical_evaluation_only() {
    let builders = read(&format!("{RUNNER}/00_builders_and_authority.rs"));
    let implementation = read(&format!("{RUNNER}/00c_day_input_builder_impl.rs"));

    assert!(builders.contains("snow_stage3_historical_evaluation_state"));
    assert!(!builders.contains("snow_stage3_persistent_state:"));
    assert!(builders.contains("constitutive Stage-3/V11\n    /// attachment owns its own"));
    assert!(implementation.contains("snow_stage3_historical_evaluation_state"));
}

#[test]
fn typed_attachment_excludes_rejected_live_carrier_and_rate_surfaces() {
    let attachment = read(&format!("{ORCHESTRATOR}/snow_stage3_v11_attachment.rs"));

    for forbidden in [
        "TerminalStateRates",
        "event_day_index",
        "event_lane_index",
        "event_elapsed_ns",
        "format!(\"{:?}",
        "wind_as_conductance",
    ] {
        assert!(
            !attachment.contains(forbidden),
            "typed attachment contains rejected surface {forbidden}"
        );
    }
    assert!(attachment.contains("evaluate_stage3_persistent_support"));
    assert!(attachment.contains("execute_direct_v11_segment"));
    assert!(attachment.contains("terminal_parcels_from_event"));
    assert!(attachment.contains("DirectV11RealConsumerStack"));
}
