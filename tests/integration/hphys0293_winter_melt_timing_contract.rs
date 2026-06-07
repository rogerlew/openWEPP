use std::fs;

const SC_SNOWFREEZE: &str = "docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md";
const SC_RUNOFFPART: &str = "docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md";
const SC_WATBAL: &str = "docs/specifications/science-contracts/contracts/SC-WATBAL-001.md";
const RUNNER_SOURCE: &str = "crates/openwepp-runner/src/hillslope/mod.rs";
const KERNEL_HELPER_SOURCE: &str =
    "crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs";
const HPHYS0284_TEST_SOURCE: &str =
    "tests/integration/hphys0284_negative_melt_snowpack_state_contract.rs";

#[test]
fn hphys0293_contracts_define_snow_producer_depletion_attribution() {
    let snow = fs::read_to_string(SC_SNOWFREEZE).expect("snow contract should be readable");
    let runoff = fs::read_to_string(SC_RUNOFFPART).expect("runoff contract should be readable");
    let watbal = fs::read_to_string(SC_WATBAL).expect("watbal contract should be readable");

    assert!(
        snow.contains("INV-SNOWFREEZE-026")
            && snow.contains("winter melt magnitude/timing and snowpack depletion invariant")
            && (snow.contains(
                "corrected `/workdir/wepp-forest` negative-melt carried-state authority"
            ) || snow.contains("fixed-comparator negative-melt carried-state authority")),
        "SC-SNOWFREEZE must define HPHYS0293 snow producer depletion authority"
    );
    assert!(
        runoff.contains("INV-RUNOFFPART-023")
            && runoff.contains("post-WB14 runoff exclusion invariant")
            && runoff.contains("residual `Q` is zero"),
        "SC-RUNOFFPART must define HPHYS0293 WB14 exclusion authority"
    );
    assert!(
        watbal.contains("INV-WATBAL-068")
            && watbal.contains("snow producer versus post-ingress storage attribution invariant")
            && watbal.contains("without authorizing empirical compensation in WB18/WB19/WB17"),
        "SC-WATBAL must define HPHYS0293 snow-before-storage attribution authority"
    );
}

#[test]
fn hphys0293_runner_trace_preserves_term_level_snow_depletion_evidence() {
    let runner = fs::read_to_string(RUNNER_SOURCE).expect("runner source should be readable");

    for required_field in [
        "snow_runtime_swe_before_m",
        "snow_runtime_swe_m",
        "snow_runtime_swe_delta_m",
        "snow_runtime_depth_before_m",
        "snow_runtime_depth_m",
        "snow_runtime_density_before_kg_m3",
        "snow_runtime_density_kg_m3",
        "snow_runtime_settle_day_count",
        "snow_s_m",
        "snow_routed_melt_m",
        "snow_post_winter_rain_m",
        "snow_hourly_rain_retained_sum_m",
        "snow_hourly_rain_released_sum_m",
        "snow_hourly_snowfall_depth_sum_m",
        "snow_hourly_snowfall_water_equiv_sum_m",
        "snow_hourly_melt_sum_m",
        "snow_hourly_melt_raw_sum_m",
        "snow_hourly_melt_amelt_in",
        "snow_hourly_melt_bmelt_in",
        "snow_hourly_melt_cmelt_in",
        "snow_hourly_melt_dmelt_in",
        "snow_runtime_swe_closure_error_m",
        "wb13_rm_mm",
        "wb13_snow_water_mm",
    ] {
        assert!(
            runner.contains(required_field),
            "runner trace must preserve HPHYS0293 snow depletion field {required_field}"
        );
    }
}

#[test]
fn hphys0293_preserves_single_source_negative_melt_boundary() {
    let helpers =
        fs::read_to_string(KERNEL_HELPER_SOURCE).expect("kernel helper source should be readable");
    let hphys0284_test =
        fs::read_to_string(HPHYS0284_TEST_SOURCE).expect("HPHYS0284 test should be readable");

    assert!(
        helpers.contains("fn redistribute_daily_signed_snowmelt")
            && helpers.contains("routed_melt_total_m: positive_melt_total_m")
            && helpers.contains("snowpack_state_loss_m: positive_melt_total_m"),
        "snow redistribution must single-source routed liquid and pack loss from authoritative positive storage loss"
    );
    assert!(
        hphys0284_test.contains("expected_state_loss = raw_positive_melt")
            && hphys0284_test.contains("does_not_undo_positive_loss_when_daily_net_is_nonpositive"),
        "HPHYS0284 regression authority must document the SNOWSCI-S1 negative-melt boundary"
    );
}

#[test]
fn hphys0293_preserves_wb14_exclusion_evidence_surfaces() {
    let runner = fs::read_to_string(RUNNER_SOURCE).expect("runner source should be readable");

    for required_field in [
        "wb12_partition_liquid_supply_m",
        "wb12_partition_residual_before_q_m",
        "wb12_infiltration_m",
        "wb13_q_mm",
        "wb14_effective_conductivity_m_s",
        "wb14_matric_potential_m",
    ] {
        assert!(
            runner.contains(required_field),
            "runner trace must preserve HPHYS0293 WB14 exclusion field {required_field}"
        );
    }
}
