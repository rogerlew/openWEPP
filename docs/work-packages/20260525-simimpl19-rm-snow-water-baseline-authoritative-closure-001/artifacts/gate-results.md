# gate-results

Status: partial
Evidence mode: ran
Date: 2026-05-25

## Static
- Required gate set for full promotability includes:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`

## Ran
- `cargo fmt --check` pass.
- Targeted contract tests pass:
  - `simimpl18_contract_requires_cold_day_partition_zero_rm_and_runtime_snow_storage`
  - `simimpl18_contract_requires_multi_day_storage_state_mutation`
- Not run in this closeout wave:
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`
