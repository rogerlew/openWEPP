# Contract-Test Implementation Evidence

Status: complete

Evidence mode: ran

Static:

- Added `tests/integration/hphys0311_snow_carry_source_line_parity_contract.rs`.
- Registered `hphys0311_snow_carry_source_line_parity_contract` in
  `Cargo.toml`.
- Test coverage checks contract authority, autonomous package scope, runner
  fail-closed source-line behavior, negative source-line fixture, and exact
  ledger counts.
- The negative source-line fixture invokes the required repo-local
  `.venv/bin/python` interpreter and fails if it is missing.
- Post-review test coverage asserts generated source-lineage requirements,
  inherited depth and density deltas for prior-year terminal holds, and
  previous-hour depth/density threshold evidence for fixed-observe precision
  holds.

Ran:

- `cargo test --test hphys0311_snow_carry_source_line_parity_contract -- --nocapture`
  initially passed with `5` tests.
- Post-verification hardening rerun:
  `cargo test --test hphys0311_snow_carry_source_line_parity_contract -- --nocapture`
  passed with `6` tests.
