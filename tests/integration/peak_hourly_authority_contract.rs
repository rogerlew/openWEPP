use std::fs;
use std::path::PathBuf;

fn repo_root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
}

fn read(path: &str) -> String {
    fs::read_to_string(repo_root().join(path)).expect("contract source must be readable")
}

#[test]
fn contract_owns_hourly_peak_return_timing_and_public_units() {
    let contract = read("docs/specifications/science-contracts/contracts/SC-WATBAL-001.md");
    for required in [
        "`INV-WATBAL-102`",
        "`INV-WATBAL-103`",
        "`INV-WATBAL-104`",
        "TOL-WATBAL-009",
        "peakro_depth = max_h(q_hourly(h)/Δt)",
        "No daily-return",
        "synthetic uniform fallback",
        "enter the WB14 hourly supply exactly once",
        "same `Area` used to convert runoff depth to `runvol`",
        "rectangular-equivalent duration",
        "GAP-WATBAL-005",
        "closed — superseded",
        "wb16_ealpha_seed_policy=retired_not_applicable",
    ] {
        assert!(
            contract.contains(required),
            "SC-WATBAL-001 is missing hourly peak authority: {required}"
        );
    }
}

#[test]
fn production_peak_consumes_shared_hourly_shape_and_depth_rate_units() {
    let runoff = read("crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs");
    for required in [
        "hourly_peak_runoff_depth_rate_m_s",
        "peak_runoff_rate_m_s",
        "DC01_HOUR_BIN_SECONDS",
        "dc01_surface_runoff_hourly_weights",
        "peak_hour_index",
    ] {
        assert!(
            runoff.contains(required),
            "production peak source is missing hourly authority marker: {required}"
        );
    }
    assert!(
        !runoff.contains("let (method_branch, qpstar) =\n            direct_peak_runoff_branch"),
        "the rainfall-envelope APPMTH branch still carries the production peak"
    );
    assert!(
        !runoff.contains("snow_reconstructed_same_pass_infiltration_m"),
        "daily-only snow infiltration reconstruction can retime hourly runoff"
    );

    let runner = read("crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs");
    assert!(
        runner.contains("WB16_EALPHA_SEED_POLICY_RETIRED_NOT_APPLICABLE"),
        "retired ealpha manifest provenance must not claim a runtime producer"
    );
    assert!(!runner.contains("WB16_EALPHA_SEED_POLICY_RUNTIME_PROVIDED"));
}

#[test]
fn publication_applies_area_once_and_erosion_consumes_depth_rate() {
    let publication =
        read("crates/openwepp-hillslope-orchestrator/src/direct_runtime/01_publication.rs");
    assert!(
        publication.contains("peak_runoff_rate_m_s * area_m2"),
        "public volumetric peak must apply area exactly once"
    );
    assert!(
        publication.contains("runvol_basis_m"),
        "peak publication must retain the event-runoff volume basis"
    );

    let erosion = read("crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion.rs");
    assert!(
        erosion.contains("peak_runoff_rate_m_s"),
        "erosion must consume the internal depth-rate peak"
    );
}

#[test]
fn hbp_consumer_contract_names_maximum_hour_and_rectangular_duration() {
    let contract = read("docs/specifications/science-contracts/contracts/SC-INFILE-HBP-001.md");
    for required in [
        "maximum hourly mean volumetric discharge",
        "max(event.hourly_runoff_volume_m3[hour]) / 3600 s",
        "Rectangular-equivalent runoff duration",
        "not physical rainfall duration, hydrograph duration, or time to peak",
    ] {
        assert!(
            contract.contains(required),
            "SC-INFILE-HBP-001 is missing corrected peak semantics: {required}"
        );
    }
}
