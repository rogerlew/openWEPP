# CQR06 Coverage Closure

Evidence class: Ran

Before target LCOV:

- Lines hit/found: `921 / 1408`
- Line coverage: `65.41%`
- Functions hit/found: `5 / 13`
- Function coverage: `38.46%`

After target LCOV:

- Lines hit/found: `1698 / 2122`
- Line coverage: `80.02%`
- Functions hit/found: `79 / 87`
- Function coverage: `90.80%`

Coverage disposition:

- Coverage did not regress; it improved by `14.61` percentage points.
- Science-tier line threshold `>= 90%` is not met.
- This package is a code-quality refactor package, not a module-test enhancement
  package. The below-threshold result is recorded as a scoped coverage hold.

Artifacts:

- `lcov_before.info`
- `lcov_after.info`
- `coverage_before.json` exists from the before run, but the useful target
  coverage evidence for this package is LCOV-backed. The before JSON was
  incomplete for the target file after an initial coverage-mode report issue.
