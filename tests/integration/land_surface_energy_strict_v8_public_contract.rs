//! External acceptance surface for the sole public V8/LSE runtime endpoint.

use std::collections::BTreeMap;
use std::fs;

use openwepp_hillslope_orchestrator::land_surface_energy_shadow::{
    LandSurfaceEnergyRealHydrologyAdapter, OfeId, Sha256Digest, SoilThermalLayerSnapshot,
    SoilThermalOfeSnapshot, SoilThermalSnapshot, SourceId, SurfaceClass, SurfaceId,
    WaterSourceType, unified_beginning_hydrology_snapshot_sha256,
};
use openwepp_hillslope_orchestrator::vegetation_real_hydrology_shadow::{
    RealHydrologyLaneLayerMap, RealHydrologyOfeLaneId, RealHydrologyShadowAdapter,
    RealHydrologySourceKey,
};
use openwepp_hillslope_orchestrator::{
    DirectGroundIngressMode, DirectOfeWb14Parameters, DirectRunFrame, DirectRunIdentity,
    DirectSubsurfaceLayerState, DirectSurfaceLiquidConfiguration,
    DirectSurfaceLiquidConfigurationRecord, DirectSurfaceLiquidOfeBinding,
    DirectSurfaceLiquidOwnedState, DirectSurfaceLiquidStoreKey,
};
use openwepp_kernel_contract::{ResourceOwnerId, SoilLayerId, TileId, TransactionId};

fn digest(byte: char) -> Sha256Digest {
    Sha256Digest::try_new(byte.to_string().repeat(64)).expect("digest")
}

#[path = "land_surface_energy_real_hydrology_shadow_contract/covered_forest_tests.rs"]
mod covered_forest_tests;

#[test]
fn raw_shadow_boundaries_remain_crate_private() {
    let workspace = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let source = fs::read_to_string(
        workspace
            .join("crates/openwepp-hillslope-orchestrator/src/land_surface_energy_shadow/mod.rs"),
    )
    .expect("land-surface shadow source");
    for name in [
        "execute_open_bare_soil_shadow",
        "execute_unified_real_hydrology_shadow",
    ] {
        assert!(
            source.contains(&format!("pub(crate) fn {name}")),
            "{name} must remain available to crate-internal boundary tests"
        );
        assert!(
            !source.contains(&format!("pub fn {name}")),
            "{name} must not become a public test bypass"
        );
        assert!(
            !source.contains(&format!("pub use {name}")),
            "{name} must not be publicly re-exported"
        );
    }
    let public_reexports = source
        .split("pub use openwepp_land_surface_energy::{")
        .nth(1)
        .and_then(|tail| tail.split("};").next())
        .expect("public LSE reexport block");
    for name in [
        "evaluate_open_surface",
        "finalize_covered_phase",
        "finalize_open_phase",
        "solve_covered_potential_phase",
        "solve_open_potential_phase",
    ] {
        assert!(
            !public_reexports.contains(name),
            "{name} must not be publicly re-exported"
        );
    }
}
