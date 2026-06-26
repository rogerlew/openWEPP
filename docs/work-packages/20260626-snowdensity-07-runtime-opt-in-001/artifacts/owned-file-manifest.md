# Owned File Manifest

Evidence class: Static.

Primary implementation:

- `crates/openwepp-hillslope-orchestrator/src/hydrology/09_snow_density.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/mod.rs`
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/infiltration_reconciliation.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/runoff_reconciliation.rs`
- `crates/openwepp-hillslope-orchestrator/src/winter_column.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/00_core_frames.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs`
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs`
- `crates/openwepp-runner/src/hillslope/snowbench_coe_melt.rs`

Tests:

- `tests/integration/snowdensity07_runtime_opt_in.rs`
- `tests/integration/snowdensity03_physics_bulk_offline_contract.rs`
- Existing SNOWDENSITY contract guards updated to `contract_version: 86`.
- R7G snow direct-runtime tests updated for explicit CoE boundary carry.

Docs:

- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/planning/snow-frost-fidelity-strategy.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260626-snowdensity-07-runtime-opt-in-001/`

