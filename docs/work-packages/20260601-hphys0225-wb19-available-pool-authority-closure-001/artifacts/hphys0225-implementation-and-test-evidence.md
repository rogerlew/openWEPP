# HPHYS0225 Implementation and Test Evidence

Status: completed  
Evidence mode: Static + Ran

## Production Remediation

File: `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`

1. Lateral phase:
   - replaced `available_pool = layer_pool.max(drainable_storage_legacy + recharge_pe)`
   - with `available_pool = layer_pool`.
2. Drainage phase:
   - replaced `available_pool = layer_pool.max(drainable_storage_legacy)`
   - with `available_pool = layer_pool`.

## Runtime Contract Tests

- Ran:
  - `cargo test --test hphys0225_wb19_layer_pool_withdrawal_cap_contract`
- Result:
  - `3 passed; 0 failed`.

## Workspace Gates

- Ran:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`
- Result:
  - all commands passed (`cargo deny` warnings only, exit success).

## Closure Measure Mapping

- `MEASURE-HP225-004`: satisfied (runtime legacy max-reconciliation removed).
- `MEASURE-HP225-005`: satisfied (HPHYS0225 contract tests pass).
- `MEASURE-HP225-006`: satisfied (required workspace gates pass).
