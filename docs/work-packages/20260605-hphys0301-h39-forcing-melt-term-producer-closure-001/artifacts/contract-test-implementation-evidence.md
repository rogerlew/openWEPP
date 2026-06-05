# Contract-Test Implementation Evidence

Status: completed

Evidence mode: static + ran

Static:

- Added `tests/integration/hphys0301_h39_forcing_melt_term_producer_contract.rs`.
- Registered the focused test in `Cargo.toml` as `hphys0301_h39_forcing_melt_term_producer_contract`.
- Test coverage asserts:
  - canonical contract authority for `INV-SNOWFREEZE-032` and `INV-WATBAL-076`,
  - package and kickoff prompt require implementation-or-blocker closure,
  - runner and ledger keep H39 first-2013 in no-production-edit `HOLD`,
  - raw-rain residual is material while released-plus-post-rain residual is sub-millimeter.

Ran:

- `cargo test --test hphys0301_h39_forcing_melt_term_producer_contract`
  - Result: pass.
  - Tests: 3 passed, 0 failed.
