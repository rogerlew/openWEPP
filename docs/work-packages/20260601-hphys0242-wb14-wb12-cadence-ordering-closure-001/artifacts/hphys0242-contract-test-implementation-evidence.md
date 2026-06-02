# HPHYS0242 Contract-Test Implementation Evidence

Status: complete
Evidence mode: Static + Ran

## Static

- `tests/integration/wb11_hydrology_kernel_contract.rs` now asserts the
  canonical hourly tail order `ET -> Drainage -> Lateral -> Runoff -> Storage`.
- `tests/integration/wb14_infiltration_hyetograph_kernel_contract.rs` now
  verifies WB14 runoff adds current-pass `ui_SCrunf(ii)` saturation carry.
- `tests/integration/wb12_reconciliation_kernel_contract.rs` now verifies WB12
  storage reconciliation depends on same-pass runoff after the WB19 tail.
- `tests/integration/wb17_et_physics_kernel_contract.rs` now verifies ET
  executes after same-pass percolation and before the WB19 tail.
- `tests/integration/wb18_percolation_physics_kernel_contract.rs` now verifies
  hourly percolation precedes final-hour ET.
- `tests/integration/wb19_lateral_drainage_physics_kernel_contract.rs` now
  verifies drainage runs before lateral and publishes saturation carry.

## Ran

- `cargo test --test wb11_hydrology_kernel_contract hphys0242 -- --nocapture`
  passed after implementation.
- `cargo test --test wb14_infiltration_hyetograph_kernel_contract hphys0242 -- --nocapture`
  passed after implementation.
- `cargo test --test wb19_lateral_drainage_physics_kernel_contract hphys0242 -- --nocapture`
  passed after implementation.
- `cargo test --test wb12_reconciliation_kernel_contract hphys0242 -- --nocapture`
  passed after implementation.
- `cargo test --test wb17_et_physics_kernel_contract hphys0242 -- --nocapture`
  passed after implementation.
- `cargo test --test wb18_percolation_physics_kernel_contract hphys0242 -- --nocapture`
  passed after implementation.
