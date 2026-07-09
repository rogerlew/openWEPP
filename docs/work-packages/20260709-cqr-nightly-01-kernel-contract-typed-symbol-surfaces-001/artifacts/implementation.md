# Implementation

Status: `COMPLETE`

Static: implementation edits are limited to:

- `crates/openwepp-kernel-contract/src/lib_mod/core_types/01_typed_symbol_surfaces.rs`
- `tests/integration/arch22_typed_state_surface_contract.rs`

Implementation summary:

- Removed the `#[allow(clippy::too_many_lines)]` attribute from
  `impl From<HillslopeProductionStateSymbol> for BoundarySymbol`.
- Replaced the single large hillslope state-symbol match with an exhaustive
  top-level category match that delegates to private helpers:
  - `hillslope_wb11_state_symbol`
  - `hillslope_wb12_state_symbol`
  - `hillslope_irrigation_scalar_state_symbol`
  - `hillslope_plant_hyetograph_soil_state_symbol`
  - `hillslope_snow_frost_state_symbol`
  - `hillslope_peak_method_state_symbol`
- Preserved dynamic `IrrigationDepletionPeriod` and
  `IrrigationFixedDateEvent` formatting in the top-level conversion.
- Added explicit invariant comments on helper `_ => unreachable!` arms. These
  arms remain private category-invariant guards behind the exhaustive top-level
  caller match, not runtime fallback wrappers or ADR-0021 coverage exclusions.
- Added characterization coverage for the touched symbol surfaces, including
  remaining climate forcing accessor, watershed channel, and watershed
  hillslope particle-diameter formats.

Behavior-preservation notes:

- No symbol literal was intentionally changed.
- No public type, variant, or method signature was changed.
- No science formula, threshold, serialization format, fail-closed behavior, or
  output semantic was changed.
