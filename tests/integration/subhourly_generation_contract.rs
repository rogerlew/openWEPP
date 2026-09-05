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
        "contract_version: 5",
        "`INV-WAT5-001`",
        "`INV-WAT5-007`",
        "`INV-WAT5-008`",
        "`INV-WAT5-009`",
        "`TOL-WAT5-001",
        "exact multiples of `300 s`",
        "mutates no water, erosion, transfer, routing, or persistent state",
        "Positive additional supply without exact typed accepted segment timing/receipt",
        "event_ordinal = 0",
        "omitted leading and trailing bins are exact zero",
        "water_only_no_erosion_adoption",
        "depression_storage_retention_depth_mm",
        "raw_wb14_post_depression_generation_depth_mm",
        "manifest as the last",
        "never claims discharge, peak, routed flow, or erosion adoption",
        "`INV-WAT5-010`",
        "`OBL-WAT5-P-005`",
        "Wat5AdditionalSupplySegmentV1",
        "Wat5AdditionalSupplySourceKindV1",
        "`SnowTerminalReceiver`",
        "`RoutedRunon`",
        "`LitterPhaseOverflow`",
        "Precipitation-\nlineage sources are prohibited from this enum",
        "i_wb14,j = checked(i_rain,j + i_add,j)",
        "sum(R5+A5)=sum(F5+D5+G5raw)",
        "The same physical mass cannot appear on two surfaces",
        "1-, 10-, and 19-OFE required case",
        "`INV-WAT5-011`",
        "`OBL-WAT5-P-006`",
        "`TOL-WAT5-002",
        "Bounded Source-Supported Closing Reconciliation Amendment",
        "`INV-WAT5-012`",
        "`OBL-WAT5-P-007`",
        "Accepted SurfaceLiquid Receipt-Complete Source Amendment",
        "must not execute a second day-wide Green-Ampt solve",
    ] {
        assert!(
            output.contains(required),
            "missing WAT5 output authority: {required}"
        );
    }
}

#[test]
fn version_three_binds_exact_typed_additional_segments_and_piecewise_replay() {
    let output = read("docs/specifications/science-contracts/contracts/SC-OUTPUT-WAT5-001.md");
    for required in [
        "## Exact Accepted Additional-Segment Source Amendment",
        "source_receipt_sha256",
        "finite `0<=start_s<end_s<=86400`",
        "The canonical segment sum must reconstruct the accepted per-source and hourly",
        "sorted union of rain boundaries, segment boundaries, and\n300-second bin boundaries",
        "WB14 advances exactly once on `i_wb14,j`",
        "rain only in `R5`, additional sources only in\n`A5`",
        "aggregate-only and partial frost-retention inputs still fail `WAT5-E-001`",
        "rainfall_and_exact_typed_additional_segments_saturation_hourly_zero_order_hold",
    ] {
        assert!(
            output.contains(required),
            "missing v3 authority: {required}"
        );
    }

    let rain_depth_m = 0.000_6_f64;
    let snow_terminal_depth_m = 0.000_3_f64;
    let routed_runon_depth_m = 0.000_15_f64;
    let litter_overflow_depth_m = 0.000_05_f64;
    let combined_supply_m =
        rain_depth_m + snow_terminal_depth_m + routed_runon_depth_m + litter_overflow_depth_m;
    let infiltration_m = 0.000_7_f64;
    let depression_m = 0.000_1_f64;
    let generation_m = combined_supply_m - infiltration_m - depression_m;
    assert_eq!(
        (infiltration_m + depression_m + generation_m).to_bits(),
        combined_supply_m.to_bits()
    );

    let index = read("docs/specifications/science-contracts/index.md");
    let row = index
        .lines()
        .find(|line| line.starts_with("| `SC-OUTPUT-WAT5-001` |"))
        .expect("WAT5 registry row");
    assert!(row.contains("v5 projects receipt-complete Stage-3 sources"));
}

#[test]
fn version_five_binds_receipt_complete_stage3_source_and_disposition_projection() {
    let output = read("docs/specifications/science-contracts/contracts/SC-OUTPUT-WAT5-001.md");
    for required in [
        "## Accepted SurfaceLiquid Receipt-Complete Source Amendment",
        "projected once from the exact sealed SurfaceLiquid ingress receipts",
        "populate `F5`, `D5`, and `G5raw`",
        "must reconstruct the accepted WB14 owner",
        "must not execute a second day-wide Green-Ampt solve",
        "`CondensationOverflow`",
        "`BEI-WAT5-004`",
        "`OBL-WAT5-P-007`",
    ] {
        assert!(
            output.contains(required),
            "missing v5 receipt-complete WAT5 authority: {required}",
        );
    }
}

#[test]
fn version_four_binds_bounded_latest_source_piece_closing_reconciliation() {
    let output = read("docs/specifications/science-contracts/contracts/SC-OUTPUT-WAT5-001.md");
    for required in [
        "## Bounded Source-Supported Closing Reconciliation Amendment",
        "epsilon_h = checked(B_h-R_h)",
        "exact `B_h>0`, exact `R_h=0`",
        "epsilon_h<=TOL-WAT5-002",
        "j* = arg max(end_s,start_s)",
        "C5(k*)       = epsilon_h",
        "G5closed(k)  = C5(k)",
        "`R5`, `A5`, `F5`, `D5`, and `G5raw` remain\nbit-identical",
        "sum(R5+A5)=sum(F5+D5+G5raw)",
        "sum(G5closed)=B_h",
        "not rainfall, additional supply, raw runoff, or a debit\nfrom a particular parcel",
        "First-positive, uniform, rainfall-shaped, largest-source, caller-last, or\nsource-free placement is prohibited",
        "2.9989032090949053e-19 m",
    ] {
        assert!(
            output.contains(required),
            "missing v4 authority: {required}"
        );
    }

    let accepted_hour_m = 2.998_903_209_094_905_3e-19_f64;
    let litter_24_29_m = 2.716_349_146_228_403e-7_f64;
    let litter_30_35_m = 4.672_156_126_607_075e-7_f64;
    let supply_m = 6.0 * litter_24_29_m + 6.0 * litter_30_35_m;
    let raw_infiltration_m = supply_m;
    let raw_depression_m = 0.0_f64;
    let raw_generation_m = 0.0_f64;
    assert_eq!(
        (raw_infiltration_m + raw_depression_m + raw_generation_m).to_bits(),
        supply_m.to_bits()
    );
    let epsilon_m = accepted_hour_m - raw_generation_m;
    let tolerance_m = 1.0e-12_f64
        * 1.0_f64
            .max(supply_m)
            .max(raw_infiltration_m)
            .max(accepted_hour_m);
    assert!(epsilon_m > 0.0 && epsilon_m <= tolerance_m);
    assert_eq!(epsilon_m.to_bits(), accepted_hour_m.to_bits());
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
        "depression_storage_retention_depth_m",
        "raw_wb14_post_depression_generation_depth_m",
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
