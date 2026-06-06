# Implementation And Test Evidence

Status: complete

Evidence mode: mixed `Static:` and `Ran:`

Static:

- Production edit:
  `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs`
  - Added `SIMIMPL28_DAILY_RADIATION_BOUND_ALLOWED`.
  - Tightened `sunmap.r3` to require `> 0 and finite`.
  - Added a fail-closed check in `simimpl28_sunmap`: if finite `radly > r3`,
    return `RuntimeContextSymbolOutOfRange { symbol: "radly", ... }`.
  - Left the existing hourly `hradmj` physical-bound guard in place.
- Test edit:
  `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
  - Added WBVAL02 daily-source regression.
  - Updated the impossible-radiation regression to fail at daily source
    `radly`.
- Contract edit:
  `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
  version `23`.

Ran:

- `cargo test -p openwepp-hillslope-orchestrator wbval02_rejects_daily_radiation_above_baseline_sunmap_potential -- --nocapture`
  passed.
- `cargo test -p openwepp-hillslope-orchestrator climate_runtime_surface_with_context_rejects_physically_impossible_radiation -- --nocapture`
  passed.
- `cargo test -p openwepp-hillslope-orchestrator climate_runtime_surface_with_context -- --nocapture`
  passed `7` tests.
- `cargo test -p openwepp-hillslope-orchestrator` passed `101` tests.
- `cargo build --release -p openwepp-runner --bin openwepp-cli-hill` passed.
  Updated binary SHA-256:
  `6aa3a88c6acfb6b57fa409a7073c755ca7ee866f449df0111a194f0e01435628`.

Safety:

- No radiation clipping, capping, guard loosening, snowmelt compensation,
  percolation compensation, or WAT-ledger compensation was introduced.
