# HPHYS0240 Contract-Test Implementation Evidence

Status: completed
Evidence mode: Static + Ran

Static: added contract-derived tests before production code edits:

- `tests/integration/wb14_infiltration_hyetograph_kernel_contract.rs`
  - `hphys0240_contract_wb14_runoff_carryover_flux_overrides_stale_runon_state`
  - `hphys0240_contract_wb14_rejects_non_finite_runoff_carryover_flux`
- `tests/integration/wb12_reconciliation_kernel_contract.rs`
  - `hphys0240_contract_wb12_storage_tail_uses_q_from_same_pass_carryover_flux`
- `tests/integration/wb11_hydrology_kernel_contract.rs`
  - `hphys0240_contract_wb11_carryover_tail_requires_storage_after_runoff`

Ran: pre-production red gate:

- `cargo test --test wb14_infiltration_hyetograph_kernel_contract hphys0240_contract -- --nocapture`
  - Result: failed as expected before implementation.
  - Evidence: WB14 stale-state probe observed `Q = 0.0900689067348141` instead of carryover-flux-derived `0.290068906744067`; malformed carryover flux was ignored and scheduler did not halt at `RunoffReconciliation`.
- `cargo test --test wb12_reconciliation_kernel_contract hphys0240_contract -- --nocapture`
  - Result: failed as expected before implementation.
  - Evidence: WB12 tail probe observed `Q = 0.2999999999999998` instead of same-pass carryover-flux-derived `0.5`.
- `cargo test --test wb11_hydrology_kernel_contract hphys0240_contract -- --nocapture`
  - Result: passed.
  - Evidence: scheduler dependency contract already preserves `Drainage -> RunoffReconciliation -> StorageReconciliation`.
