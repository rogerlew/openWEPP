# Gate Results

Status: complete
Evidence mode: Ran

## Rust Gates

| Gate | Result | Notes |
| --- | --- | --- |
| `cargo fmt --check` | passed | Final run in chained gate. |
| `cargo clippy --workspace --all-targets -- -D warnings` | passed | Local Clippy findings fixed before final pass. |
| `cargo test --workspace` | passed | Workspace tests and doctests passed. |
| `cargo deny check` | passed | Non-failing duplicate crate and unmatched license allowance warnings emitted by current deny posture. |
| `wctl doc-lint --path <package>` | passed | Tool reported `0 files validated, 0 errors, 0 warnings`. |
| `wctl doc-lint --path docs/work-packages/README.md` | passed | Tool reported `1 files validated, 0 errors, 0 warnings`. |

## Semantic Gates

| Gate | Result | Notes |
| --- | --- | --- |
| Full H1..H39 runtime | passed | `/tmp/hphys0283_full3_20260604T163035Z`; `39/39` runtime completion. |
| Full H1..H39 semantic reports | passed as evidence run | Semantic pass remains `0/39`; metrics recorded for continuation. |
| H1/H7/H39 targeted traces | passed | `/tmp/hphys0283_springtrace3_20260604T164525Z`; all three CLI runs returned `0`. |

## Focused Tests

- `clim05_snow_runtime_kernel_contract`: passed.
- `hphys0283_snowmelt_infiltration_partition_contract`: passed.
- `wb14_infiltration_hyetograph_kernel_contract`: passed.
- `wb12_reconciliation_kernel_contract`: passed.
- `wb18_percolation_physics_kernel_contract`: passed.
- `sim_contract_boundary_unit_registry`: passed.
