use std::fs;

fn repo_text(path: &str) -> String {
    fs::read_to_string(path).unwrap_or_else(|error| panic!("failed to read {path}: {error}"))
}

#[test]
fn contract_binds_density_process_ledger_without_promotion() {
    let contract =
        repo_text("docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md");
    for binding in [
        "INV-SNOWFREEZE-087",
        "OBL-SNOWFREEZE-P-061",
        "TOL-SNOWFREEZE-012",
        "behavior-neutral",
        "diagnostic-only",
        "fresh-snow density",
        "destructive metamorphism",
        "overburden compaction",
    ] {
        assert!(
            contract.contains(binding),
            "missing contract binding: {binding}"
        );
    }
}

#[test]
fn runtime_and_real_consumer_expose_non_aliased_process_fields() {
    let density =
        repo_text("crates/openwepp-hillslope-orchestrator/src/hydrology/09_snow_density.rs");
    let partition = repo_text(
        "crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs",
    );
    let consumer = repo_text(
        "crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs",
    );
    for field in [
        "initial_snow_mass_kg_m2",
        "liquid_for_compaction_mass_kg_m2",
        "compaction_temperature_c",
        "fresh_snow_density_kg_m3",
        "fresh_snow_mixing_delta_kg_m3",
        "wet_compaction_delta_kg_m3",
        "destructive_metamorphism_delta_kg_m3",
        "overburden_compaction_delta_kg_m3",
        "structural_projection_delta_kg_m3",
        "climate_fallback_used",
        "climate_fallback_delta_kg_m3",
        "internal_cap_delta_kg_m3",
        "runtime_cap_delta_kg_m3",
        "downstream_stage3_delta_kg_m3",
        "closure_residual_kg_m3",
    ] {
        assert!(density.contains(field), "density runtime missing {field}");
        assert!(
            consumer.contains(&format!("density_process_{field}")),
            "real consumer missing {field}"
        );
    }
    assert!(partition.contains("density_process_diagnostics"));
    assert!(consumer.contains("openwepp-r7h-direct-production-snow-trace-v4"));
}
