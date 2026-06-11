# Line Count Governance Checklist

Status: complete

Evidence mode: Static

Date: 2026-06-11

## Diff Shape

Static:

- `git diff --stat` after the layered-store continuation reports 26 modified
  existing files with 840 insertions and 327 deletions across production frost,
  contract tests, canonical contracts, and package evidence. Seven generated
  layered cohort report artifacts are also staged as new files, bringing the
  staged diff to 33 files, 1343 insertions, and 327 deletions.
- No new production modules were introduced.
- The largest touched Rust files are pre-existing package surfaces; FDHP01
  edits were localized to existing frost, WAT publication, runner fixture, and
  unit-catalog seams.

## Touched File Counts

Representative post-edit line counts:

- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling.rs`: 1082
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`: 337
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_runoff_reconciliation.rs`: 1078
- `crates/openwepp-hillslope-output/src/hillslope_wat.rs`: 722
- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`: 1425
- `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs`: 1606
- `crates/openwepp-runner/src/hillslope/03_tests.rs`: 570
- `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_publication.rs`: 314
- `crates/openwepp-runner/src/hillslope/tests03/trace.rs`: 1230
- `crates/openwepp-sim-contract/src/units_mod/boundary_catalog.rs`: 1425
- `crates/openwepp-sim-contract/src/units_mod/output_catalog.rs`: 1321
- `tests/integration/clim06_frost_frozen_soil_kernel_contract.rs`: 1201
- `tests/integration/cli04_runner_wat_parquet_contract_derived_tests.rs`: 254
- `tests/integration/sim_contract_boundary_unit_registry.rs`: 804
- `tests/integration/hphys0319_fixed_baseline_stmtim_observe_contract.rs`: 231
- `tests/integration/hphys0320_stmtim_start_time_source_line_contract.rs`: 149
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`: 876
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`: 2422
- `docs/specifications/subsystems/runner/openwepp-hillslope-cli-specification.md`: 304

## Disposition

No unrelated split/refactor was performed. Several files exceed preferred
small-file size already, but splitting them would cross FDHP01's correction
scope and increase review risk. The package kept edits narrow and contract-led.
