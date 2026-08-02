use std::fs;

fn repo_text(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("failed to read {path}: {error}"))
}

#[test]
fn contract_binds_phase_accumulation_and_melt_diagnostics_without_promotion() {
    let contract =
        repo_text("docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md");
    for binding in [
        "contract_version: 121",
        "INV-SNOWFREEZE-088",
        "OBL-SNOWFREEZE-P-062",
        "TOL-SNOWFREEZE-013",
        "snowfall SWE",
        "amelt",
        "bmelt",
        "cmelt",
        "dmelt",
        "physical site redistribution remains unknown",
        "diagnostic-only",
    ] {
        assert!(
            contract.contains(binding),
            "missing contract binding: {binding}"
        );
    }
}

#[test]
fn typed_runtime_and_real_consumer_expose_required_non_aliased_fields() {
    let forcing = repo_text(
        "crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs",
    );
    let melt = repo_text(
        "crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/infiltration_reconciliation.rs",
    );
    let projection = repo_text(
        "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00a_snow_frost_authority_impl.rs",
    );
    let consumer = repo_text(
        "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs",
    );
    let trace_helper = repo_text(
        "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00f_snow_accumulation_melt_trace.rs",
    );
    for field in [
        "active_precipitation_m",
        "rain_fraction",
        "snow_fraction",
        "phase_model",
        "hydrometeor_temperature_c",
        "snowfall_swe_m",
        "coe_melt_amelt_m",
        "coe_melt_bmelt_m",
        "coe_melt_cmelt_m",
        "coe_melt_dmelt_m",
        "coe_melt_uncapped_m",
        "coe_melt_cap_adjustment_m",
        "coe_melt_applied_m",
        "modeled_wind_redistribution_m",
    ] {
        assert!(
            forcing.contains(field) || melt.contains(field),
            "typed runtime missing {field}"
        );
        assert!(
            projection.contains(field) || consumer.contains(field) || trace_helper.contains(field),
            "real projection/consumer missing {field}"
        );
    }
    assert!(consumer.contains("openwepp-r7h-direct-production-snow-trace-v3"));
}
