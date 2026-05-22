# Slope Runtime Builder Implementation Evidence

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Added slope runtime projection API `build_hillslope_runtime_surface_from_slope`.
- Added slope seam helper guards and symbol synthesis helpers:
  - `validate_slope_profile_shape`
  - `validate_slope_ofe_shape`
  - `validate_slope_points`
  - `derive_avgslp`
  - symbol mappers for OFE/point keys and primary aliases
- Expanded `HillslopeRuntimeInputError` with `HS-RUNTIME-E-011..025` for slope seam guard failures.
- Added unit tests in orchestrator runtime input module for slope projection happy path and typed error path.
- Added integration coverage in `tests/integration/parser_runtime_seam_integration.rs` for slope seam closure and guard rejection.

Ran:
- Full workspace gates executed successfully after implementation.

## Implementation Summary

Primary code changes:
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
  - new slope builder implementation at `:439`
  - slope seam guard helpers at `:760` and below
  - unit tests at `:1057` and `:1109`
- `tests/integration/parser_runtime_seam_integration.rs`
  - slope runtime probe kernel at `:94`
  - slope seam closure integration test at `:206`
  - slope seam guard failure integration test at `:232`

Behavior introduced:
- OFE-count and point-count structural closure enforcement.
- Finite/domain checks for `slplen`, `xinput`, `slpinp`.
- Explicit monotonic `xinput` runtime guard.
- Typed derived `avgslp` calculation with explicit rejection of non-finite or non-positive results.
- Canonical + indexed symbol surface projection for multi-OFE slope profiles.

## Integration Test Evidence

New and updated coverage (Ran):
- `runtime_inputs::tests::slope_runtime_surface_contains_canonical_state_symbols`
- `runtime_inputs::tests::slope_runtime_surface_rejects_non_positive_derived_avgslp`
- `slope_parser_to_hillslope_runtime_surface_closure`
- `slope_runtime_surface_rejects_non_positive_avgslp_projection`

Observed `cargo test --workspace` excerpt for this package surface:
- Integration target `tests/integration/parser_runtime_seam_integration.rs` reported `9 passed; 0 failed`.
- Unit target `openwepp_hillslope_orchestrator` reported `25 passed; 0 failed`.

## Parity/Closure Notes

Static:
- `avgslp` derivation follows legacy `profil.for` trapezoidal profile integration shape, but runtime seam policy intentionally emits typed errors for non-positive/non-finite derived values instead of legacy silent clamp.

Ran:
- All required SR02 gates completed with pass status; `cargo deny check` returned only allowlist hygiene warnings (`license-not-encountered`) and final status `advisories ok, bans ok, licenses ok, sources ok`.
