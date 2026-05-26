# SIMIMPL28 Contract-Test Implementation Evidence

Status: complete
Evidence mode: static+ran
Date: 2026-05-25

## Static
- SIMIMPL28 contract-derived tests were implemented in
  `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs` and exercise:
  - hourly forcing symbol emission across all 24 hours,
  - `rst`-controlled rain/snow partition branch behavior,
  - typed hard-fail behavior when required winter context is missing.
- Slope runtime projection tests were updated to assert `azm` publication,
  supporting required forcing synthesis geometry inputs.

## Ran
- `cargo test -p openwepp-hillslope-orchestrator climate_runtime_surface_with_context_`
- `cargo test -p openwepp-hillslope-orchestrator slope_runtime_surface_contains_canonical_state_symbols`
- `rg -n "climate_runtime_surface_with_context_emits_simimpl28_hourly_forcing_symbols|climate_runtime_surface_with_context_respects_rst_partition_branches|climate_runtime_surface_with_context_rejects_missing_required_winter_symbol|slope_runtime_surface_contains_canonical_state_symbols" crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
