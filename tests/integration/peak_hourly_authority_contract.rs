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
    assert!(
        erosion.contains("DIRECT_EROD13_DURATION_CUSTODY_TOLERANCE_S: f64 = 1.001e-9"),
        "erosion must name the absolute seconds duration custody tolerance"
    );
    assert!(
        erosion.contains("> DIRECT_EROD13_DURATION_CUSTODY_TOLERANCE_S"),
        "the live duration guard must consume its seconds-specific tolerance"
    );

    let contract = read("docs/specifications/science-contracts/contracts/SC-SED-001.md");
    for required in [
        "peakro_depth = max_h(V_h / Area / 3600 s)",
        "peakro = peakro_depth · Area",
        "rectangular-equivalent duration",
        "TOL-SED-009",
        "abs(watdur - Q / peakro_depth) <= 1.001e-9 s",
        "matching the active erosion guard",
        "It is not a sediment-continuity tolerance",
        "never the public volumetric value or a separate analytical estimator",
        "no uniform or rainfall-window fallback is authorized",
    ] {
        assert!(
            contract.contains(required),
            "SC-SED-001 is missing corrected erosion peak authority: {required}"
        );
    }
    for retired in [
        "max(V_h/3600) ≠ peakro",
        "wb16_tstar",
        "wb16_qpstar",
        "wb16_vstar",
        "watdur = Q/peakro`",
        "watdur - (Q / peakro_depth)) <= TOL-SED-001",
        "1e-9 s * max(1, abs(watdur), abs(Q / peakro_depth))",
    ] {
        assert!(
            !contract.contains(retired),
            "SC-SED-001 retains retired erosion peak authority: {retired}"
        );
    }

    let adr = read("docs/decisions/0036-hydrograph-resolved-sediment-transport-and-routing.md");
    for required in [
        "production WB16 peak is the maximum hourly mean",
        "peakro_depth = max_h(q_hourly(h) / 3600 s)",
        "`peakro` is not an independent analytical estimator",
        "retain only an explicit compatibility fallback",
    ] {
        assert!(
            adr.contains(required),
            "ADR-0036 is missing reconciled hourly-peak authority: {required}"
        );
    }
    for retired in [
        "WB16 `peakro` is a **separate analytical\nestimator**",
        "`max(V_h/3600) ≠ peakro` is **not** an error",
        "Rescale the hourly profile so `max(hourly) = peakro`",
    ] {
        assert!(
            !adr.contains(retired),
            "ADR-0036 retains contradicted independent-peak authority: {retired}"
        );
    }
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
