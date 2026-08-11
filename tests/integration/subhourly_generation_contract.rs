use std::fs;
use std::path::PathBuf;

fn read(path: &str) -> String {
    fs::read_to_string(PathBuf::from(env!("CARGO_MANIFEST_DIR")).join(path))
        .expect("contract/source must be readable")
}

#[test]
fn canonical_contracts_bind_five_minute_water_without_changing_peak() {
    let watbal = read("docs/specifications/science-contracts/contracts/SC-WATBAL-001.md");
    for protected in ["`INV-WATBAL-102`", "`INV-WATBAL-103`", "`INV-WATBAL-104`"] {
        assert!(
            watbal.contains(protected),
            "peak authority was lost: {protected}"
        );
    }

    let output = read("docs/specifications/science-contracts/contracts/SC-OUTPUT-WAT5-001.md");
    for required in [
        "`INV-WAT5-001`",
        "`INV-WAT5-007`",
        "`TOL-WAT5-001",
        "exact multiples of `300 s`",
        "mutates no water, erosion, transfer, routing, or persistent state",
        "Positive additional supply without five-minute timing",
        "event_ordinal = 0",
        "omitted leading and trailing bins are exact zero",
        "water_only_no_erosion_adoption",
        "never claims discharge, peak, routed flow, or erosion adoption",
    ] {
        assert!(
            output.contains(required),
            "missing WAT5 output authority: {required}"
        );
    }
}

#[test]
fn production_has_one_typed_post_wb14_wb19_generation_ledger() {
    let runtime =
        read("crates/openwepp-hillslope-orchestrator/src/direct_runtime/subhourly_generation.rs");
    for required in [
        "DirectFiveMinuteGenerationInterval",
        "DirectFiveMinuteGenerationEvent",
        "WAT5_INTERVAL_SECONDS",
        "closed_wb14_generation_depth_m",
        "saturation_return_depth_m",
        "closing_surface_generation_depth_m",
        "water_only_no_erosion_adoption",
    ] {
        assert!(
            runtime.contains(required),
            "missing runtime ledger marker: {required}"
        );
    }

    let executor = read("crates/openwepp-hillslope-orchestrator/src/direct_runtime/03_executor.rs");
    let peak = executor
        .find("run_r7d6_peak_runoff_span")
        .expect("peak span exists");
    let diagnostic = executor
        .find("run_wat5_subhourly_generation")
        .expect("WAT5 diagnostic span exists");
    let storage = executor
        .find("run_r4b_storage_reconciliation_span")
        .expect("storage span exists");
    assert!(
        peak < diagnostic && diagnostic < storage,
        "WAT5 must run after closing peak and before later publication spans"
    );
}
