# Contract-Test Implementation Evidence

Status: complete

Evidence mode: static + ran

Static:

- Added `tests/integration/hphys0303_adr0016_comparator_ratification_contract.rs`.
- Registered test target `hphys0303_adr0016_comparator_ratification_contract`
  in `Cargo.toml`.
- Test asserts local-only ratification gates, fixed comparator identity,
  no remote push, ledger/ADR status coherence, HPHYS0302 HOLD carry-forward,
  SC lint pass gating, parquet year/key validation, scoped observe identity,
  and explicit smoke-helper disposition.

Ran:

- `cargo test --test hphys0303_adr0016_comparator_ratification_contract -- --nocapture`:
  pass, 3 tests.
- `cargo test --test hphys0302_comparator_surface_audit_contract -- --nocapture`:
  pass, 3 tests.
