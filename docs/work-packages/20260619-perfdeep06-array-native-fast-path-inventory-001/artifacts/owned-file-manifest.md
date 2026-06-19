# PERFDEEP06 Owned-File Manifest

Status: complete 2026-06-19.
Evidence class: Static.

## Writable Files

- `docs/work-packages/20260619-perfdeep06-array-native-fast-path-inventory-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`
- `docs/architecture/array-native-runtime-specification.md` only for small
  consistency corrections discovered during planning.

## Read-Only Source Inventory

- `crates/openwepp-hillslope-orchestrator/src/day_frame.rs`
- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
- `crates/openwepp-hillslope-orchestrator/src/phase.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/state_access.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/04_kernel_execution.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/**`
- `crates/openwepp-hillslope-orchestrator/src/constants.rs`
- `crates/openwepp-kernel-contract/src/lib_mod/core_types/00_symbol_registry_and_indexed_surfaces.rs`
- `crates/openwepp-kernel-contract/src/lib_mod/core_types/02_boundary_values_and_kernel_requests.rs`
- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
- `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs`
- `crates/openwepp-runner/src/hillslope/scheduler_trace/**`
- `crates/openwepp-hillslope-output/src/hillslope_wat.rs`
- `crates/openwepp-hillslope-output/src/hillslope_pass.rs`

## Scope Rule

No production Rust code is writable unless `package.md` is amended before
implementation with an explicit bounded write set, contract-first gates, and
review of the new acceptance criteria.

Execution followed this rule. Only package artifacts and roadmap/catalog/spec
docs were modified.
