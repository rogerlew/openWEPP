# Contract Test Implementation Evidence

Status: complete

Evidence mode: Static

Static:

- Added `tests/integration/hphys0317_hourly_snowfall_input_surface_parity_contract.rs`.
- Registered the test in `Cargo.toml`.
- Refreshed the HPHYS0316 regression test to assert contract authority without
  pinning obsolete exact contract-version numbers after HPHYS0317 advanced the
  same canonical contracts.
- The test asserts:
  - `SC-CLIMATE-001#INV-CLIMATE-015`
  - `SC-SNOWFREEZE-001#INV-SNOWFREEZE-043`
  - `SC-WATBAL-001#INV-WATBAL-091`
  - combined `57` carried-row ledger status,
  - HPHYS0318 follow-on ownership,
  - no production edit authorization,
  - validation command records and final artifact states.
