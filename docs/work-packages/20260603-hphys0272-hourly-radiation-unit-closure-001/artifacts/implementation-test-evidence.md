# Implementation Test Evidence

Status: completed
Evidence mode: static + ran

Static:

- Production edit:
  `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs`
  now treats daily climate `day.rad` as `radly` and computes
  `radmj = radly * 0.04184` before `hr_tmp`.
- `sunmap` still consumes `radly`; the fix does not clip radiation, change
  snowmelt terms, or compensate WB13/WB17/storage residuals.

Ran:

- `cargo test -p openwepp-hillslope-orchestrator climate_runtime_surface_with_context_uses_single_radly_to_radmj_conversion --lib -- --nocapture`:
  passed.
- `cargo test -p openwepp-hillslope-orchestrator climate_runtime_surface_with_context_near_isothermal_radiation_is_radmj_over_24 --lib -- --nocapture`:
  passed.
- `cargo test -p openwepp-hillslope-orchestrator climate_runtime_surface_with_context_emits_simimpl28_hourly_forcing_symbols --lib -- --nocapture`:
  passed.
- `cargo test -p openwepp-hillslope-orchestrator runtime_inputs::tests --lib`:
  `47 passed`.
