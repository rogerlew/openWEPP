# Line Count Governance Checklist

Status: executed-hold

Evidence mode: Static

Date: 2026-06-12

## Diff Shape

Static:

- `git diff --stat` for D3 Increment De reports 13 tracked
  code/contract/test/package files with 520 insertions and 20 deletions before
  the new De evidence artifacts are counted.
- No new production modules were introduced.
- The largest touched Rust files are pre-existing package surfaces; FDHP01
  edits remain localized to the existing frost coupling seam, contract-test,
  version-expectation, contract, and package-evidence surfaces.

## Touched File Counts

Representative post-edit line counts:

- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling.rs`: 2749
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/state_access.rs`: 1897
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/02_soil_slope.rs`: 1702
- `tests/integration/clim06_frost_frozen_soil_kernel_contract.rs`: 2515
- `tests/integration/hphys0319_fixed_baseline_stmtim_observe_contract.rs`: 231
- `tests/integration/hphys0320_stmtim_start_time_source_line_contract.rs`: 149
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`: 989

## Disposition

No unrelated split/refactor was performed. `coupling.rs` remains above the
2000-line WARN threshold at 2749 lines, and the CLIM06 contract-test file is
also above the WARN threshold at 2515 lines. Splitting either file inside De
would cross the lower-front heat-flow correction scope and increase review
risk, so the WARN is accepted for this increment. The next frost-side
localization pass should continue treating seam extraction and test-file
partitioning as planned cleanup once the executable frost state-machine
behavior is stable. No touched non-exempt Rust file reaches the 3000-line
refactor-blocking threshold.
