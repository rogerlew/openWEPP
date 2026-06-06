# Contract-Test Implementation Evidence

Status: complete

Evidence mode: mixed `Static:` and `Ran:`

Static:

- Added
  `runtime_inputs::tests::wbval02_rejects_daily_radiation_above_baseline_sunmap_potential`.
  The fixture uses the WBVAL02 DRIGGS metadata and the first source-bound
  offending rows:
  - `1990-02-18`, `radly=486 Ly d^-1`
  - `1990-02-19`, `radly=503 Ly d^-1`
- Updated
  `runtime_inputs::tests::climate_runtime_surface_with_context_rejects_physically_impossible_radiation`
  to require source-symbol `radly` for a finite impossible daily radiation
  payload.
- Preserved existing unit-lineage regressions:
  - `climate_runtime_surface_with_context_uses_single_radly_to_radmj_conversion`
  - `climate_runtime_surface_with_context_near_isothermal_radiation_is_radmj_over_24`

Ran:

- Pre-implementation red gate:
  - `cargo test -p openwepp-hillslope-orchestrator wbval02_rejects_daily_radiation_above_baseline_sunmap_potential -- --nocapture`
    failed before production edits because the runtime surface built instead of
    failing at source `radly`.
  - `cargo test -p openwepp-hillslope-orchestrator climate_runtime_surface_with_context_rejects_physically_impossible_radiation -- --nocapture`
    failed before production edits because the error symbol was still
    `winter.hourly.rad_mj_m2_0009`.
- Post-implementation green gate:
  - `cargo test -p openwepp-hillslope-orchestrator wbval02_rejects_daily_radiation_above_baseline_sunmap_potential -- --nocapture`
    passed.
  - `cargo test -p openwepp-hillslope-orchestrator climate_runtime_surface_with_context_rejects_physically_impossible_radiation -- --nocapture`
    passed.
  - `cargo test -p openwepp-hillslope-orchestrator climate_runtime_surface_with_context -- --nocapture`
    passed `7` targeted climate-runtime tests.
