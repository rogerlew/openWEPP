# CQR28 Coverage Closure

Ran: before and after LCOV files are stored as package artifacts:

- `artifacts/lcov_before.info`
- `artifacts/lcov_after.info`

Target-file LCOV:

| Metric | Before | After |
|---|---:|---:|
| Lines hit/found | `790/1149` | `971/1331` |
| Line coverage | `68.76%` | `72.95%` |
| Functions hit/found | `10/14` | `23/27` |
| Function coverage | `71.43%` | `85.19%` |

Closure: target-file line and function coverage did not regress.

Characterization basis:

- Ran pre-refactor focused suite:
  `cargo test --test wb18_percolation_physics_kernel_contract`.
- No new characterization tests were needed before production extraction
  because the existing WB18 contract suite covers the lane, restrictive layer,
  same-pass infiltration, guard, and layerwise flux branches selected for
  helper extraction.
- During extraction, the focused suite caught a bottom-layer percolation-loss
  accumulator regression. The refactor was corrected to preserve accumulated
  bottom flux across upper-layer routes, and the focused suite passed after the
  fix.
