use std::fs;

const SC_SNOWFREEZE: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const SC_RUNOFFPART: &str = "docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md";
const SC_WATBAL: &str = "docs/specifications/science-contracts/contracts/SC-WATBAL-001.md";
const RUNNER_SOURCE: &str = "crates/openwepp-runner/src/hillslope/mod.rs";
const KERNEL_HELPER_SOURCE: &str =
    "crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs";

#[test]
fn hphys0296_contracts_define_snow_rm_acceptance_authority() {
    let snow = fs::read_to_string(SC_SNOWFREEZE).expect("snow contract should be readable");
    let runoff = fs::read_to_string(SC_RUNOFFPART).expect("runoff contract should be readable");
    let watbal = fs::read_to_string(SC_WATBAL).expect("watbal contract should be readable");

    assert!(
        snow.contains("INV-SNOWFREEZE-027")
            && snow.contains("snow/`RM` producer acceptance invariant")
            && snow.contains(
                "necessary diagnostic evidence but are not sufficient acceptance authority"
            )
            && snow.contains("per-window defective-model verdict")
            && snow.contains("reconstruction controlled experiment")
            && snow.contains("independent correctness adjudication")
            && snow.contains("documented-legacy-defective")
            && snow.contains("Downstream WB17/WB18/WB19/WB13 compensation is invalid"),
        "SC-SNOWFREEZE must define HPHYS0296 snow/RM producer acceptance authority"
    );
    assert!(
        runoff.contains("INV-RUNOFFPART-024")
            && runoff.contains("runoff-consumer acceptance invariant")
            && runoff.contains("closed runoff-consumer identity is necessary evidence only")
            && runoff.contains("per-window defective-model verdict")
            && runoff.contains("runoff partitioning remains excluded"),
        "SC-RUNOFFPART must keep closed runoff partitioning separate from snow/RM producer residuals"
    );
    assert!(
        watbal.contains("INV-WATBAL-071")
            && watbal.contains("snow/`RM` acceptance invariant")
            && watbal.contains("may not by themselves accept a residual")
            && watbal.contains("per-window defective-model verdict")
            && watbal.contains("No WB17/WB18/WB19/WB13 downstream compensation is allowed"),
        "SC-WATBAL must define the HPHYS0296 downstream-compensation prohibition"
    );
}

#[test]
fn hphys0296_runner_trace_preserves_snow_rm_acceptance_surfaces() {
    let runner = fs::read_to_string(RUNNER_SOURCE).expect("runner source should be readable");

    for required_field in [
        "snow_runtime_swe_before_m",
        "snow_runtime_swe_m",
        "snow_runtime_swe_delta_m",
        "snow_runtime_depth_before_m",
        "snow_runtime_depth_m",
        "snow_runtime_density_before_kg_m3",
        "snow_runtime_density_kg_m3",
        "snow_s_m",
        "snow_routed_melt_m",
        "snow_post_winter_rain_m",
        "snow_hourly_rain_retained_sum_m",
        "snow_hourly_rain_released_sum_m",
        "snow_hourly_snowfall_water_equiv_sum_m",
        "snow_hourly_melt_sum_m",
        "snow_hourly_melt_raw_sum_m",
        "snow_hourly_melt_raw_m",
        "snow_runtime_swe_closure_error_m",
        "wb12_infiltration_m",
        "wb13_q_mm",
        "wb13_rm_mm",
        "wb13_snow_water_mm",
    ] {
        assert!(
            runner.contains(required_field),
            "runner trace must preserve HPHYS0296 snow/RM acceptance surface {required_field}"
        );
    }
}

#[test]
fn hphys0296_preserves_corrected_negative_melt_not_baseline_bug_compatibility() {
    let helpers =
        fs::read_to_string(KERNEL_HELPER_SOURCE).expect("kernel helper source should be readable");
    let snow = fs::read_to_string(SC_SNOWFREEZE).expect("snow contract should be readable");

    assert!(
        helpers.contains("fn redistribute_daily_signed_snowmelt")
            && helpers.contains("routed_melt_total_m: net_melt_m")
            && helpers
                .contains("snowpack_state_loss_m: positive_melt_total_m - negative_melt_total_m")
            && helpers.contains("SIMIMPL29_SNOWPACK_STATE_LOSS_OVERDRAW_TOLERANCE_M"),
        "HPHYS0296 acceptance must preserve corrected negative-melt routed/state-loss authority"
    );
    assert!(
        snow.contains("corrected-fix derivation/provenance")
            && snow.contains("/workdir/wepp-forest_260430_baseline"),
        "HPHYS0296 contract authority must distinguish corrected semantic parity from bug-compatible baseline reproduction"
    );
}
