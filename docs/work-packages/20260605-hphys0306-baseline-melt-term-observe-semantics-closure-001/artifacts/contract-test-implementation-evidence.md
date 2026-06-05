# Contract-Test Implementation Evidence

Status: complete

Evidence mode: ran

Static:

- Added `tests/integration/hphys0306_baseline_melt_observe_semantics_contract.rs`.
- Registered the test in `Cargo.toml`.
- The test requires package/prompt branch-active semantics, canonical
  `INV-WATBAL-079`, runner no-remote behavior, and an executed nine-row ledger
  with typed branch-mask HOLD semantics.

Ran:

- `cargo test --test hphys0306_baseline_melt_observe_semantics_contract -- --nocapture` passed: `4` tests.
