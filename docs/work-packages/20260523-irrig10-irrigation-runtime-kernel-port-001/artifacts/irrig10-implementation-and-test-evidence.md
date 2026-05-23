# IRRIG10 Implementation and Test Evidence

Status: `completed`
Evidence mode: `Static + Ran`

## Production Implementation Surfaces

Implemented IRRIG10 runtime coupling in:

- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
  - added IRRIG runtime symbols/constants
  - added fixed-date + depletion event resolution helpers
  - added runtime event normalization + schedule-source precedence
  - coupled irrigation depth into WB14 rainfall/infiltration/runoff reconciliation
  - published runtime trace surfaces and `Irr` flux
  - coupled `Irr` into WB12 storage reconciliation
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
  - added irrigation depletion/fixed-date runtime-surface builders/seeding
  - added typed irrigation schedule validation and count conversion helpers
  - mapped irrigation schedule errors to `HS-RUNTIME-E-056..059`

## Test Coverage Executed

- Targeted IRRIG10 contract tests:
  - `cargo test --test irrig10_irrigation_runtime_kernel_contract -- --nocapture`
- Hydrology non-regression integrations:
  - `cargo test --test wb11_hydrology_kernel_contract --test wb12_reconciliation_kernel_contract --test wb14_infiltration_hyetograph_kernel_contract --test wb15_canopy_interception_kernel_contract --test clim05_snow_runtime_kernel_contract --test clim06_frost_frozen_soil_kernel_contract -- --nocapture`
- Full required gates (final run):
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`

## Result

All listed runs completed successfully (with only expected `cargo deny`
license-not-encountered warnings and overall `licenses ok`).
