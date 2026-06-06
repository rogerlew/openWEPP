# Contract-Test Implementation Evidence

Status: complete

Evidence mode: ran

Static:

- Added `tests/integration/hphys0309_snow_carry_depletion_lineage_contract.rs`.
- Registered the focused test in `Cargo.toml`.
- Relaxed HPHYS0308 contract-version assertion so later contract amendments do
  not break prior invariant tests.

Ran:

- `cargo fmt --check` passed.
- `cargo test --test hphys0309_snow_carry_depletion_lineage_contract -- --nocapture`
  passed.
