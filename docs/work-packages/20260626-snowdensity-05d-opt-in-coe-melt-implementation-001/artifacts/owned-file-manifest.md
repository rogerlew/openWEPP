# Owned File Manifest

Evidence class: Static.

## Work-Package Files

- `docs/work-packages/20260626-snowdensity-05d-opt-in-coe-melt-implementation-001/`

## Contract And Planning

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/planning/snow-frost-fidelity-strategy.md`
- `docs/work-packages/README.md`

## Rust Production Code

- `crates/openwepp-hillslope-orchestrator/src/hydrology/08_snow_albedo.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/infiltration_reconciliation.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_infiltration_evap.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_runoff_reconciliation.rs`
- `crates/openwepp-hillslope-orchestrator/src/winter_column.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs`

## Tests

- `Cargo.toml`
- `tests/integration/snowdensity05d_opt_in_coe_melt.rs`
- `tests/integration/snowdensity02_contract_adr_guard.rs`
- `tests/integration/snowdensity05a_melt_contract_guard.rs`
- `tests/integration/snowdensity05b_shortwave_source_contract.rs`
- `tests/integration/snowdensity05c_albedo_state_core.rs`
- Direct runtime regression fixtures under
  `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/`.
