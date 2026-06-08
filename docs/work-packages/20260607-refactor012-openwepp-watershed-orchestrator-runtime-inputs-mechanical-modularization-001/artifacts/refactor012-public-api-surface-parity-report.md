# REFACTOR012 refactor012 public api surface parity report

Status: complete  
Evidence mode: Static: completed; Ran: completed

## Scope
Static:
- Scope target: preserved runtime-input public API and re-export shape after module extraction.
- Mechanical split preserved all intent-exposed constructors and typed public error surfaces.
- No export additions were introduced for behavior or API reasons.

## Public symbol inventory (post-refactor)

Static:
- `runtime_inputs.rs` now re-exports:
  - `build_watershed_climate_runtime_request_from_assignments`
  - `build_watershed_runtime_surface_from_climate_assignments`
  - `seed_watershed_runtime_surface_from_climate`
  - `build_watershed_runtime_surface_from_chaninp`
  - `seed_watershed_runtime_surface_from_watershed_channel`
  - `seed_watershed_runtime_surface_from_slope_channel_profile`
  - `seed_watershed_runtime_surface_from_watershed_impoundment`
  - `WatershedRuntimeInputError`
  - `WatershedClimateRuntimeInputError`
  - `WatershedClimateRuntimeRequest`
  - `WatershedHillslopeClimateAssignment`
- `runtime_inputs` internals and tests moved to:
  - `runtime_inputs_mod/chaninp.rs`
  - `runtime_inputs_mod/climate.rs`
  - `runtime_inputs_mod/types.rs`
  - `runtime_inputs_mod/tests.rs`

Ran:
- 43 focused unit tests in `openwepp-watershed-orchestrator` pass after refactor.
- `cargo test --workspace` passed with 0 failures.
- Re-export compile/test behavior confirms no public API regressions in consumer call paths.
