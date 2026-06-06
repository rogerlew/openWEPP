# Contract-Test Implementation Evidence

Status: complete

Evidence mode: ran

Static:

- Added `tests/integration/hphys0308_snowd_branch_state_ordering_contract.rs`.
- Registered the test in `Cargo.toml`.

Ran:

- `cargo test --test hphys0308_snowd_branch_state_ordering_contract -- --nocapture`
  passed with `5` tests after one package-wording patch.
