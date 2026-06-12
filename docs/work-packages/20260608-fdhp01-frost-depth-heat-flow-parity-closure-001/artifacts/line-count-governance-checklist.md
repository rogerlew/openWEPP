# Line Count Governance Checklist

Status: executed-hold

Evidence mode: Static

Date: 2026-06-12

## Diff Shape

Static:

- `git diff --stat` for D3 Increment Dj reports 18 tracked
  code/contract/test/package files with 737 insertions and 73 deletions before
  the new Dj evidence artifacts are counted.
- No new production modules were introduced.
- The largest touched Rust files are pre-existing package surfaces; FDHP01
  edits remain localized to the existing frost coupling seam, runtime hourly
  forcing projection, contract-test, version-expectation, unit-registry,
  contract, and package-evidence surfaces.

## Touched File Counts

Representative post-edit line counts:

- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling.rs`: 3052
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/state_access.rs`: 1875
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs`: 897
- `tests/integration/clim06_frost_frozen_soil_kernel_contract.rs`: 2743
- `tests/integration/hphys0319_fixed_baseline_stmtim_observe_contract.rs`: 231
- `tests/integration/hphys0320_stmtim_start_time_source_line_contract.rs`: 149
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`: 1068

## Disposition

No unrelated split/refactor was performed. `coupling.rs` now crosses the
3000-line refactor-blocking threshold at 3052 lines, and the CLIM06
contract-test file remains above the 2000-line WARN threshold at 2743 lines.
Splitting either file inside Dj would cross the surface-temperature
source-line port scope and increase review risk while the frost residual is
still actively being localized. The threshold breach is accepted only as an
executed-hold carry-forward item for the next frost-side increment: extract the
legacy frost surface-temperature/thermal-resistance helpers or partition the
FDHP01 contract-test vectors before adding another large production block.
