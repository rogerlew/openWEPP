# Contract-Test Implementation Evidence

Status: complete

Evidence mode: ran

Static:

- Added `tests/integration/hphys0312_prior_year_terminal_snowpack_lineage_contract.rs`.
- Registered `hphys0312_prior_year_terminal_snowpack_lineage_contract` in
  `Cargo.toml`.
- Test coverage checks contract authority, autonomous package scope, runner
  fail-closed source-line behavior, negative source-line fixture, generated
  source-lineage requirements, exact ledger route counts, material first
  divergence, terminal continuity, and no-compensation posture.
- The negative source-line fixture invokes the required repo-local
  `.venv/bin/python` interpreter and fails if it is missing.

Ran:

- `cargo test --test hphys0312_prior_year_terminal_snowpack_lineage_contract -- --nocapture`
  passed with `6` tests.
