# Line Count Governance Checklist

Status: executed-hold

Evidence mode: Static

Date: 2026-06-12

## Diff Shape

Static:

- `git diff --stat` for D3 Increment C2 reports 5 tracked code/contract/test
  files with 538 insertions and 32 deletions before package-record updates,
  plus generated C2 evidence artifacts under this work package.
- No new production modules were introduced.
- The largest touched Rust files are pre-existing package surfaces; FDHP01
  edits remain localized to the existing frost coupling seam, contract-test,
  version-expectation, contract, and package-evidence surfaces.

## Touched File Counts

Representative post-edit line counts:

- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling.rs`: 2240
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`: 394
- `crates/openwepp-hillslope-orchestrator/src/hydrology/04_kernel_execution.rs`: 180
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_infiltration_evap.rs`: 1331
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_plant_percolation.rs`: 1300
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_runoff_reconciliation.rs`: 1218
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/hydrology.rs`: 837
- `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs`: 1663
- `crates/openwepp-runner/src/hillslope/scheduler_trace/hphys_trace.rs`: 1056
- `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs`: 1886
- `crates/openwepp-runner/src/hillslope/tests03/publication/publication_wb13_guard.rs`: 437
- `crates/openwepp-runner/src/hillslope/tests03/trace.rs`: 1274
- `tests/integration/clim06_frost_frozen_soil_kernel_contract.rs`: 2006
- `tests/integration/hphys0319_fixed_baseline_stmtim_observe_contract.rs`: 231
- `tests/integration/hphys0320_stmtim_start_time_source_line_contract.rs`: 149
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`: 983

## Disposition

No unrelated split/refactor was performed. `coupling.rs` remains above the
2000-line WARN threshold at 2240 lines, and the CLIM06 contract-test file is
also now above the WARN threshold at 2006 lines. Splitting either file inside
C2 would cross the thaw-arm/state-machine correction scope and increase review
risk, so the WARN is accepted for this increment. The next freeze-arm
energy/resistance pass should treat seam extraction and test-file partitioning
as planned cleanup once the executable frost state-machine behavior is stable.
No touched non-exempt Rust file reaches the 3000-line refactor-blocking
threshold.
