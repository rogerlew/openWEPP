#[path = "runtime_inputs_mod/mod.rs"]
mod runtime_inputs_mod;

pub use runtime_inputs_mod::chaninp::{
    build_watershed_runtime_surface_from_chaninp,
    seed_watershed_runtime_surface_from_slope_channel_profile,
    seed_watershed_runtime_surface_from_watershed_channel,
    seed_watershed_runtime_surface_from_watershed_impoundment,
};
pub(crate) use runtime_inputs_mod::chaninp::{
    derive_ws12_impoundment_coefficients, derive_ws12_outflow_function_families,
};
pub use runtime_inputs_mod::climate::{
    build_watershed_climate_runtime_request_from_assignments,
    build_watershed_runtime_surface_from_climate_assignments,
    seed_watershed_runtime_surface_from_climate,
};
pub use runtime_inputs_mod::types::{
    WatershedClimateRuntimeInputError, WatershedClimateRuntimeRequest,
    WatershedHillslopeClimateAssignment, WatershedRuntimeInputError,
};
