# HPHYS0242 Implementation and Test Evidence

Status: complete
Evidence mode: Static + Ran

## Static

- `phase.rs` and `scheduler.rs` now place `Drainage` before
  `LateralTransfer`, with `RunoffReconciliation` depending on lateral output.
- `03_kernel_support_00_support_helpers.rs` now requires explicit
  `ui_SCrunf(ii)` current-pass saturation-carry arrays when MOFE hourly carry
  arrays are enabled, while preserving the positive final-storage hard failure.
- `03_kernel_support_01_kernel_phases.rs` now computes same-pass WB14
  infiltration lineage for stage-memory ET paths, allows drainage to execute
  before lateral, publishes `Qd` after lateral when same-pass `Qdd` is present,
  publishes current `ui_SCrunf(ii)` during hourly lateral substeps, and adds
  `Σui_SCrunf(ii)` into WB14/WB12 runoff reconciliation.
- No production physics proxy or heuristic replacement was introduced; touched
  production behavior follows the amended canonical `SC-*` cadence authority.
- `crates/openwepp-runner/src/hillslope/mod.rs` was in the intended write set
  but did not require changes for this closure.

## Ran

- `cargo fmt --check && cargo clippy --workspace --all-targets -- -D warnings`
  passed after a local clippy `too_many_lines` annotation on the helper.
- Package-listed integration tests passed:
  - `cargo test --test wb11_hydrology_kernel_contract -- --nocapture`
  - `cargo test --test wb14_infiltration_hyetograph_kernel_contract -- --nocapture`
  - `cargo test --test wb12_reconciliation_kernel_contract -- --nocapture`
  - `cargo test --test wb17_et_physics_kernel_contract -- --nocapture`
  - `cargo test --test wb18_percolation_physics_kernel_contract -- --nocapture`
  - `cargo test --test wb19_lateral_drainage_physics_kernel_contract -- --nocapture`
- `cargo test --workspace` passed.
- `cargo deny check` passed with existing warning-class duplicate/unmatched
  license allowance messages and final `advisories ok, bans ok, licenses ok,
  sources ok`.
