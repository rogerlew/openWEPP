use std::fs;

const RUNNER_SOURCE: &str = "crates/openwepp-runner/src/hillslope/mod.rs";
const KERNEL_PHASE_SOURCE: &str =
    "crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs";

#[test]
fn hphys0289_contract_requires_wb13_rm_from_routed_wmelt_not_swe_delta_proxy() {
    let runner = fs::read_to_string(RUNNER_SOURCE).expect("runner source should be readable");

    assert!(
        runner.contains(
            "require_runtime_surface_scalar_prefer_flux(runtime_surface, \"snow.routed_melt_m\")"
        ),
        "WB13 publication must consume an explicit routed wmelt surface"
    );
    assert!(
        !runner.contains("precipitation_m + runtime_swe_before_m - runtime_swe_m + irrigation_m"),
        "WB13 RM must not use raw precipitation plus SWE-delta proxy math"
    );
}

#[test]
fn hphys0289_contract_requires_kernel_to_publish_daily_routed_wmelt() {
    let kernel =
        fs::read_to_string(KERNEL_PHASE_SOURCE).expect("kernel phase source should be readable");

    assert!(
        kernel.contains("BoundarySymbol::from(\"snow.routed_melt_m\")"),
        "WB12/WB14 runoff reconciliation must publish daily routed wmelt for WB13"
    );
}
