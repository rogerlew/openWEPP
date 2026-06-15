# CQR05 CRAP After

Evidence: Ran.

Commands:

- `cargo llvm-cov --workspace --ignore-run-fail --json --output-path docs/work-packages/20260615-cqr05-erod14-wave2-complexity-001/artifacts/coverage_after.json`
- `cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr05-erod14-wave2-complexity-001/artifacts/lcov_after.info`
- `cargo crap --workspace --lcov docs/work-packages/20260615-cqr05-erod14-wave2-complexity-001/artifacts/lcov_after.info --min 0 --format json --output docs/work-packages/20260615-cqr05-erod14-wave2-complexity-001/artifacts/crap_after.json`

Exit codes:

- `cargo llvm-cov` JSON: `0`
- `cargo llvm-cov` LCOV: `0`
- `cargo crap`: `0`

Highest unique target rows:

| Function | Line | Cyclomatic | Coverage | CRAP |
| --- | ---: | ---: | ---: | ---: |
| `erod14_load_raw_inputs` | 172 | 23.0 | 100.0 | 23.0 |
| `erod14_reproportion_iteration` | 745 | 9.0 | 56.41025641025641 | 15.708693673190716 |
| `run_erod14_wave2` | 88 | 14.0 | 100.0 | 14.0 |
| `erod14_push_class_state` | 511 | 13.0 | 97.36842105263158 | 13.00307989502843 |
| `erod14_case_one_matches` | 430 | 3.0 | 0.0 | 12.0 |
| `erod14_validate_flow_inputs` | 297 | 11.0 | 85.5072463768116 | 11.368330852427178 |
| `erod14_reproportion_to_ldbot` | 703 | 9.0 | 75.0 | 10.265625 |

Closure:

- Final target-file maximum CRAP: `23.0`.
- Package threshold: `<= 30`.
- Result: pass.

Note:

- The CRAP JSON contains duplicate rows for some instrumented functions. The
  closure check uses the maximum CRAP value, which is unaffected by duplicates.
