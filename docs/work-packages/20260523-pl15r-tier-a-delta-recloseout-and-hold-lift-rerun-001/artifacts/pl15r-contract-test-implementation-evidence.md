# PL15R Contract-Test Implementation Evidence

Status: `complete`
Evidence mode: `Static + Ran`

## Implemented Tests

- Added integration test target in `Cargo.toml`:
  - `pl15r_tier_a_delta_recloseout_contract`
- Added new test file:
  - `tests/integration/pl15r_tier_a_delta_recloseout_contract.rs`

## Contract-Derived Assertions

1. Tier-A single-OFE daily surface continues to route as
   `HigherConfidence` with canonical message id.
2. PL14R schema-aligned `H5.wat.dat` comparator artifact reports strict pass.
3. PL14R schema-aligned `H5.plot.dat` comparator artifact reports strict pass.
4. Day-by-day WB13 parity artifact reports exact 25-measure keyed parity
   (`all_columns_exact=true`, `common_row_count=1095`).
5. Risk-acceptance reference remains mandatory only when unresolved Tier-A
   blockers remain.

## Executed Contract Gate Command

```bash
cargo test --test pl15r_tier_a_delta_recloseout_contract -- --nocapture
```

Result: `ok` (`5 passed`, `0 failed`).
