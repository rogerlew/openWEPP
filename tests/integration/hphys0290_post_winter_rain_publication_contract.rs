use std::fs;

const RUNNER_SOURCE: &str = "crates/openwepp-runner/src/hillslope/mod.rs";
const KERNEL_PHASE_SOURCE: &str =
    "crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs";
const UNIT_REGISTRY_SOURCE: &str = "crates/openwepp-sim-contract/src/units.rs";

#[test]
fn hphys0290_contract_requires_wb13_rm_from_explicit_post_winter_rain_surface() {
    let runner = fs::read_to_string(RUNNER_SOURCE).expect("runner source should be readable");

    assert!(
        runner.contains(
            "require_runtime_flux_surface_scalar(runtime_surface, \"snow.post_winter_rain_m\")"
        ),
        "WB13 publication must consume explicit producer flux snow.post_winter_rain_m"
    );
    assert!(
        !runner.contains("reset_daily_snow_publication_fluxes"),
        "WB13 publication must not be satisfied by a reset/default state value"
    );
    assert!(
        !runner.contains("let post_winter_rain_m = if winter_partition_active"),
        "WB13 post-winter rain must not be inferred from snow-active state"
    );
    assert!(
        !runner.contains("} else { precipitation_m };"),
        "WB13 post-winter rain must not fall back to raw precipitation"
    );
}

#[test]
fn hphys0290_contract_requires_kernel_to_publish_post_winter_rain() {
    let kernel =
        fs::read_to_string(KERNEL_PHASE_SOURCE).expect("kernel phase source should be readable");

    assert!(
        kernel.contains("BoundarySymbol::from(\"snow.post_winter_rain_m\")"),
        "WB12/WB14 runoff reconciliation must publish post-winter direct rain for WB13"
    );
}

#[test]
fn hphys0290_contract_requires_unit_registry_for_post_winter_rain() {
    let units =
        fs::read_to_string(UNIT_REGISTRY_SOURCE).expect("unit registry source should be readable");

    assert!(
        units.contains("\"snow.post_winter_rain_m\""),
        "boundary unit registry must declare snow.post_winter_rain_m"
    );
    assert!(
        units.contains("SC-SNOWFREEZE-001#INV-SNOWFREEZE-023"),
        "unit registry must cite the HPHYS0290 snow publication invariant"
    );
}
