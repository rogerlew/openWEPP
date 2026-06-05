# Contract Test Implementation Evidence

Status: complete
Evidence mode: Static

Static:

- Added `tests/integration/hphys0290_post_winter_rain_publication_contract.rs` and registered it in `Cargo.toml`.
- Added source-level contract checks requiring WB13 to call `require_runtime_flux_surface_scalar(runtime_surface, "snow.post_winter_rain_m")` and forbidding the old snow-active/raw-precipitation inference branch.
- Added source-level contract checks requiring the kernel to publish `BoundarySymbol::from("snow.post_winter_rain_m")` and the unit registry to cite `SC-SNOWFREEZE-001#INV-SNOWFREEZE-023`.
- Extended `tests/integration/sim_contract_boundary_unit_registry.rs` to require `snow.post_winter_rain_m` as a canonical runtime snow alias.
- Added runner unit tests for explicit post-winter rain consumption, missing surface failure, state-only fallback rejection, negative value failure, non-finite value failure, and flux-over-state precedence.

Disposition: initial contract-derived tests were authored before production changes and failed in the pre-implementation gate; the state-only fallback regression was added during review disposition to prevent canonicalize-and-proceed masking.
