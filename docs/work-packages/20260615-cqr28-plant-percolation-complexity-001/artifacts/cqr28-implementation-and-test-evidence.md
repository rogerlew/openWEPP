# CQR28 Implementation and Test Evidence

Implementation:

- Added private structs for WB18 layer vectors, same-pass infiltration,
  percolation lane configuration, routing results, and scalar ledger context.
- Extracted WB18 percolation helpers from `run_percolation`:
  validation, layer reads, same-pass infiltration resolution, lane config,
  routing, substep routing, layer routing, formula calculation, conductivity
  adjustment, unscaled `pei`, deep-percolation roundoff canonicalization,
  soil-water ledger reconciliation, and response construction.
- Kept helper visibility private and preserved the existing public/crate-visible
  entry point.

Focused test evidence:

- Ran pre-refactor: `cargo test --test wb18_percolation_physics_kernel_contract`
  and all 16 tests passed.
- During extraction, the focused suite exposed an accumulator regression in
  substep bottom percolation loss. The helper was corrected to accumulate
  substep bottom loss instead of overwriting it.
- Ran after fix: `cargo test --test wb18_percolation_physics_kernel_contract`
  and all 16 tests passed.

Final gate evidence:

- Ran `cargo fmt --check`: passed.
- Ran `cargo clippy --workspace --all-targets -- -D warnings`: passed.
- Ran `cargo test --workspace`: passed.
- Ran `cargo deny check`: passed.
- Ran final after LCOV/CRAP: target/helper CRAP closure passed.
- Ran markdown-doc lint for package and README: passed with 22 files scanned,
  0 errors, 0 warnings.
- Ran `git diff --check`: passed.
