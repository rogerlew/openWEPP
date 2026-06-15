# CQR14 CRAP After

Ran: refreshed final after LCOV with
`cargo llvm-cov clean --workspace && cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr14-runner-release-complexity-001/artifacts/lcov_after.info`

Ran: `cargo crap --workspace --lcov docs/work-packages/20260615-cqr14-runner-release-complexity-001/artifacts/lcov_after.info --min 0 --format json --output docs/work-packages/20260615-cqr14-runner-release-complexity-001/artifacts/crap_after.json`

Static: target-file after LCOV summary:

- Lines: `488/571`, `85.46%`
- Functions: `46/59`, `77.97%`

Static: final CQR14 target row:

| Function | Line | CC | Coverage | CRAP |
| --- | ---: | ---: | ---: | ---: |
| `lint_release_directory` | 50 | 4.0 | 100.0 | 4.0 |

Static: newly extracted helper closure rows:

| Function | CC | Coverage | CRAP |
| --- | ---: | ---: | ---: |
| `collect_release_candidate_binaries` | 8.0 | 76.66666666666667 | 8.813037037037036 |
| `lint_release_binary` | 8.0 | 100.0 | 8.0 |
| `ReleaseHbpPair::record` | 4.0 | 83.33333333333334 | 4.0740740740740735 |
| `validate_lint_sidecar_role` | 3.0 | 66.66666666666666 | 3.333333333333334 |
| `lint_release_binaries` | 3.0 | 100.0 | 3.0 |
| `validate_lint_sidecar_binary_name` | 3.0 | 100.0 | 3.0 |
| `validate_release_hbp_pair` | 3.0 | 100.0 | 3.0 |
| `lint_sidecar_hbp_supported` | 2.0 | 46.666666666666664 | 2.606814814814815 |
| `validate_lint_sidecar` | 1.0 | 50.0 | 1.125 |
| `required_lint_sidecar_str` | 1.0 | 70.0 | 1.027 |

Static: maximum CRAP among the CQR14 target and newly extracted helpers is
`8.813037037037036`.

WARN: `validate_release_sidecar_unlocked` remains the highest target-file row
at CRAP `31.459079074798446`; it is pre-existing and outside the CQR14 scoped
metric target.
