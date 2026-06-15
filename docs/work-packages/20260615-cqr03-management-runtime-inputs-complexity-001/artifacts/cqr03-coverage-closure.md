# Coverage Closure

Ran: before summary coverage with
`cargo llvm-cov --workspace --ignore-run-fail --json --summary-only --output-path docs/work-packages/20260615-cqr03-management-runtime-inputs-complexity-001/artifacts/coverage_before_summary.json`.

Ran: after summary coverage with
`cargo llvm-cov --workspace --ignore-run-fail --json --summary-only --output-path docs/work-packages/20260615-cqr03-management-runtime-inputs-complexity-001/artifacts/coverage_after_summary.json`.

Both commands exited `0`.

| Measurement | Functions | Instantiations | Lines | Regions |
|---|---:|---:|---:|---:|
| Before | 10/11, 90.9090909090909% | 20/22, 90.9090909090909% | 690/954, 72.32704402515722% | 1130/1344, 84.07738095238095% |
| After | 59/60, 98.33333333333333% | 115/120, 95.83333333333334% | 1147/1220, 94.01639344262294% | 1371/1500, 91.4% |

ADR-0021 classification: this target is glue-tier runtime projection. Closure
threshold is `>= 85%` line and region.

Disposition: coverage closure passed after characterization and refactor.
