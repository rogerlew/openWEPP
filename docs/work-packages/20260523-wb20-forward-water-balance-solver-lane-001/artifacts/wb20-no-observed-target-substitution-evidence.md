# WB20 No Observed Target Substitution Evidence

Status: `completed`
Evidence mode: `Ran`

## Ran Proof Vector
- Test: `wb20_contract_conformance_forward_lane_excludes_observed_targets`
- File: `tests/integration/wb20_forward_water_balance_solver_lane_contract.rs`
- Action:
  - sets `wb20_forward_solver_lane_enabled = 1`
  - removes `wb12_runoff_observed` and `wb12_storage_observed`
- Observed result:
  - scheduler succeeds
  - `wb12_runoff_closure_delta` is near zero solver residual
  - `wb12_storage_closure_delta` is near zero solver residual

## Control Vector
- Test: `wb20_contract_conformance_compatibility_lane_keeps_observed_closure_path`
- Action:
  - sets `wb20_forward_solver_lane_enabled = 0`
  - perturbs `wb12_runoff_observed`
- Observed result:
  - typed domain failure at runoff reconciliation (`HKERNEL-WB14-RUNOFF-E-003`)

## Conclusion
Forward-solver lane acceptance no longer substitutes observed targets into
closure semantics, while compatibility lane remains explicit and unchanged.
