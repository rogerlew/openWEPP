# HPHYS0225 Runtime Source-Scan Contract Test Evidence

Status: completed
Evidence mode: Static + Ran

## Implemented test change
- Updated `tests/integration/hphys0225_wb19_layer_pool_withdrawal_cap_contract.rs` in
  `hphys0225_runtime_source_forbids_legacy_max_reconciliation`:
  - replaced monolith-only scan with recursive hydrology source scan,
  - preserved the same forbidden-expression invariants,
  - preserved the layer-derived available pool invariant assertion.

## Evidence outcome
- Ran: `cargo test --test hphys0225_wb19_layer_pool_withdrawal_cap_contract`.
- Result: pass.

## Contract intent preserved
- Forbidden `layer_pool.max(drainable_storage_legacy + recharge_pe)` remains blocked.
- Forbidden `layer_pool.max(drainable_storage_legacy)` remains blocked.
- `let available_pool = layer_pool;` remains required.
