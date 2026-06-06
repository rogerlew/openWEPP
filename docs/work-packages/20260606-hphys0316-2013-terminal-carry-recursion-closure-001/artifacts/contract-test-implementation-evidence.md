# Contract-Test Implementation Evidence

Status: complete

Evidence mode: Static + Ran

Static:

- Added `tests/integration/hphys0316_2013_terminal_carry_recursion_contract.rs`.
- Registered the test in `Cargo.toml`.
- The test asserts:
  - `SC-SNOWFREEZE-001#INV-SNOWFREEZE-042`.
  - `SC-WATBAL-001#INV-WATBAL-090`.
  - H1/H7/H39 spring-2016 carried-row counts (`15`, `9`, `9`; total `33`).
  - 2014 day 1 hour 1 continuity to matching 2013 terminal deltas.
  - The 2013 day 11 hour 11 baseline/openWEPP hourly snowfall input blocker.
  - ADR0017 verdict `UNRESOLVED`.
  - `production_edit_authorized=false`.
  - Follow-on owner `HPHYS0317`.
  - Required gate/review/verification closeout records.

Ran:

- `cargo test --test hphys0316_2013_terminal_carry_recursion_contract hphys0316_contract_authority_is_registered -- --nocapture`
  completed with exit status `0` as the pre-implementation contract gate.
