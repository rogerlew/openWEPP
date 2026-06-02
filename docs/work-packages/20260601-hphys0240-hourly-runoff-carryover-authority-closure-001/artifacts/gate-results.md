# HPHYS0240 Gate Results

Status: completed
Evidence mode: Ran

Ran: required gates:

- `cargo fmt --check`
  - Result: passed.
- `cargo clippy --workspace --all-targets -- -D warnings`
  - Result: passed.
- `cargo test --workspace`
  - Result: passed.
- `cargo deny check`
  - Result: passed with warnings.
  - Warnings: existing duplicate crate entries for `getrandom`, `hashbrown`,
    `twox-hash`, and unmatched license allowances `ISC`,
    `Unicode-DFS-2016`; command exited `0`.

Ran: targeted implementation checks:

- `cargo test --test wb14_infiltration_hyetograph_kernel_contract hphys0240_contract -- --nocapture`
  - Result: passed.
- `cargo test --test wb12_reconciliation_kernel_contract hphys0240_contract -- --nocapture`
  - Result: passed.
- `cargo test --test wb11_hydrology_kernel_contract hphys0240_contract -- --nocapture`
  - Result: passed.
- `cargo test --test wb14_infiltration_hyetograph_kernel_contract && cargo test --test wb12_reconciliation_kernel_contract && cargo test --test wb11_hydrology_kernel_contract`
  - Result: passed.
