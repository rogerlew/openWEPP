# gate-results

Status: complete
Evidence mode: ran
Date: 2026-05-25

## Static
- Required SIMIMPL18 gate set executed on current package state.

## Ran
- `cargo fmt --check` -> pass (`rc=0`)
- `cargo clippy --workspace --all-targets -- -D warnings` -> pass (`rc=0`)
- `cargo test --workspace` -> fail (`rc=101`)
- `cargo deny check` -> pass (`rc=0`)
- Workspace test failure is isolated to SIMIMPL18 contract tests:
  - `simimpl18_contract_requires_cold_day_partition_zero_rm_and_runtime_snow_storage`
  - `simimpl18_contract_requires_multi_day_storage_state_mutation`
- Logs:
  - `artifacts/replay-run-20260525T132822Z/gates/gate_exit_codes.log`
  - `artifacts/replay-run-20260525T132822Z/gates/fmt.stdout.log`
  - `artifacts/replay-run-20260525T132822Z/gates/clippy.stdout.log`
  - `artifacts/replay-run-20260525T132822Z/gates/test.stdout.log`
  - `artifacts/replay-run-20260525T132822Z/gates/deny.stdout.log`
