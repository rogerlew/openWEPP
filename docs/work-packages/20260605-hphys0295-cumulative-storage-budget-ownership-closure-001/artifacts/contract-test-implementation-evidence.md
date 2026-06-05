# Contract-Test Implementation Evidence

Status: executed
Evidence mode: Static + Ran

Static:
- Added `tests/integration/hphys0295_cumulative_storage_budget_contract.rs`.
- Registered the integration test in `Cargo.toml`.
- Contract-derived checks require:
  - `SC-WATBAL-001#INV-WATBAL-070`.
  - `SC-EVAP-001#INV-EVAP-027`.
  - Cross-contract anchors to `SC-PERC-001#INV-PERC-019` and
    `SC-SUBHYD-001#INV-SUBHYD-031`.
  - Runner trace observability for storage, ET, percolation, lateral,
    runoff, snow, and `RM` budget terms.

Ran:
- `cargo test --test hphys0295_cumulative_storage_budget_contract -- --nocapture`
- Result: passed, `3 passed; 0 failed`.
