# HPHYS0242 Gate Results

Status: complete
Evidence mode: Ran

## Ran

- `cargo test --test wb11_hydrology_kernel_contract hphys0242 -- --nocapture`
  - Result: passed after implementation.
- `cargo test --test wb14_infiltration_hyetograph_kernel_contract hphys0242 -- --nocapture`
  - Result: passed after implementation.
- `cargo test --test wb19_lateral_drainage_physics_kernel_contract hphys0242 -- --nocapture`
  - Result: passed after implementation.
- `cargo test --test wb12_reconciliation_kernel_contract hphys0242 -- --nocapture`
  - Result: passed after implementation.
- `cargo test --test wb17_et_physics_kernel_contract hphys0242 -- --nocapture`
  - Result: passed after implementation.
- `cargo test --test wb18_percolation_physics_kernel_contract hphys0242 -- --nocapture`
  - Result: passed after implementation.
- `cargo test --test wb11_hydrology_kernel_contract -- --nocapture`
  - Result: passed; 12 passed, 0 failed.
- `cargo test --test wb14_infiltration_hyetograph_kernel_contract -- --nocapture`
  - Result: passed; 11 passed, 0 failed.
- `cargo test --test wb12_reconciliation_kernel_contract -- --nocapture`
  - Result: passed; 5 passed, 0 failed.
- `cargo test --test wb17_et_physics_kernel_contract -- --nocapture`
  - Result: passed; 5 passed, 0 failed.
- `cargo test --test wb18_percolation_physics_kernel_contract -- --nocapture`
  - Result: passed; 11 passed, 0 failed.
- `cargo test --test wb19_lateral_drainage_physics_kernel_contract -- --nocapture`
  - Result: passed; 8 passed, 0 failed.
- `cargo fmt --check && cargo clippy --workspace --all-targets -- -D warnings`
  - Result: passed after local clippy annotation.
- `cargo test --workspace`
  - Result: passed.
- `cargo deny check`
  - Result: passed; warning output reported duplicate `getrandom`,
    `hashbrown`, and `twox-hash` lock entries plus unmatched license
    allowances for `ISC` and `Unicode-DFS-2016`.
- `wctl doc-lint --path docs/work-packages/20260601-hphys0242-wb14-wb12-cadence-ordering-closure-001`
  - Result: passed; reported 0 configured files validated.
- `wctl doc-lint --path docs/work-packages/README.md`
  - Result: passed; 1 file validated, 0 errors, 0 warnings.
