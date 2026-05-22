# Soil Runtime Builder Implementation Evidence

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Expanded `HillslopeRuntimeInputError` with SR03 soil seam guard variants (`HS-RUNTIME-E-026..035`).
- Reworked `build_hillslope_runtime_surface_from_soil` to iterate all OFEs/layers and emit indexed layer/profile symbols.
- Added symbol helper mappers for OFE and layer key synthesis.
- Added and updated unit/integration tests for canonical projection and typed saturated-conductivity guard failure.

Ran:
- Full required SR03 gate set completed successfully after implementation.

## Implementation Summary

Primary code changes:
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
  - expanded seam builder and guards at `:514`
  - OFE/layer symbol projection and first-OFE aliases at `:587`
  - soil symbol helper functions at `:1245`
  - soil seam unit tests at `:1358` and `:1445`
- `tests/integration/parser_runtime_seam_integration.rs`
  - expanded runtime closure assertions for `nsl`, `ssc`, indexed layer symbols at `:52`
  - new integration failure-path test for missing `ksat_mm_h` at `:211`

Behavior introduced:
- Runtime seam now exports `ntemp`, per-OFE `nsl`/`solthk`, per-layer `solthk/dg/thetdr/thetfc/ssc`, and first-OFE alias surfaces.
- `ssc` projection uses parser `ksat_mm_h` converted from `mm/h` to `m/s`.
- Strict typed rejection for malformed OFE/layer shape and missing/non-finite/non-positive conductivity inputs.

## Integration Test Evidence

Ran:
- `parser_to_hillslope_runtime_surface_closure` verifies scheduler-consumable soil runtime projection including:
  - `nsl = 2`
  - `ssc = 15.0/3.6e6`
  - `ssc_0002 = 8.0/3.6e6`
- `soil_runtime_surface_rejects_missing_saturated_conductivity_projection` verifies typed failure:
  - error code `HS-RUNTIME-E-033`
  - variant `MissingSaturatedConductivity { ofe_index: 1, layer_index: 1 }`
- `runtime_inputs::tests::soil_runtime_surface_contains_canonical_state_symbols` verifies unit-level projection for canonical and indexed symbols.
- `runtime_inputs::tests::soil_runtime_surface_rejects_missing_saturated_conductivity` verifies unit-level typed guard behavior.

## Parity/Closure Notes

Static:
- Layer `ssc` values are sourced from `ksat_mm_h` in parser layer rows (`valid_9002.sol`: first layer `15.0`, second layer `8.0`) and not from texture percentages.
- Seam scope remains parser-to-runtime boundary projection; downstream dynamic conductivity fields (`Ksi`, `Ksai`) remain consumer/kernel responsibilities.

Ran:
- Workspace tests validated both seam-closure and typed-failure paths with passing results.
