# Contract Test Implementation Evidence

Status: completed
Evidence mode: static + ran

Static:

- Added `climate_runtime_surface_with_context_uses_single_radly_to_radmj_conversion`
  in `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`.
- Added
  `climate_runtime_surface_with_context_near_isothermal_radiation_is_radmj_over_24`
  in `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`.
- Tests cover both the active `sunmap`/`radcur` hourly branch and the exact
  near-isothermal `radmj / 24` branch.

Ran:

- Pre-implementation red gate:
  `cargo test -p openwepp-hillslope-orchestrator climate_runtime_surface_with_context_uses_single_radly_to_radmj_conversion --lib -- --nocapture`
  failed with `max=30.504601472037276`, `daily_radmj=8.368`.
- Post-implementation:
  `cargo test -p openwepp-hillslope-orchestrator climate_runtime_surface_with_context_uses_single_radly_to_radmj_conversion --lib -- --nocapture`
  passed.
- Post-implementation:
  `cargo test -p openwepp-hillslope-orchestrator climate_runtime_surface_with_context_near_isothermal_radiation_is_radmj_over_24 --lib -- --nocapture`
  passed.
