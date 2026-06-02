# HPHYS0240 Implementation and Test Evidence

Status: completed
Evidence mode: Static + Ran

Static: production implementation:

- Added `WB12_SYMBOL_RUNOFF_CARRYOVER = "wb12_runoff_carryover"` in
  `crates/openwepp-hillslope-orchestrator/src/constants.rs`.
- Added symbol-based optional flux reading and
  `resolve_runoff_carryover_input` in
  `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`.
- Updated `run_runoff_reconciliation` in
  `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
  to prefer same-pass `wb12_runoff_carryover`, validate malformed present
  fluxes, fall back to `wb12_runon_input` only when flux is absent, and publish
  resolved carryover as a flux with `Q`.
- Seeded runner runtime surface flux `wb12_runoff_carryover = 0.0` in
  `crates/openwepp-runner/src/hillslope/mod.rs`.

Ran: focused HPHYS0240 tests after implementation:

- `cargo test --test wb14_infiltration_hyetograph_kernel_contract hphys0240_contract -- --nocapture`
  - Result: passed, 2/2.
- `cargo test --test wb12_reconciliation_kernel_contract hphys0240_contract -- --nocapture`
  - Result: passed, 1/1.
- `cargo test --test wb11_hydrology_kernel_contract hphys0240_contract -- --nocapture`
  - Result: passed, 1/1.

Ran: full modified integration files:

- `cargo test --test wb14_infiltration_hyetograph_kernel_contract`
  - Result: passed, 10/10.
- `cargo test --test wb12_reconciliation_kernel_contract`
  - Result: passed, 4/4.
- `cargo test --test wb11_hydrology_kernel_contract`
  - Result: passed, 9/9.
