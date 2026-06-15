# CQR05 CRAP Before

Evidence: Ran.

Commands:

- `cargo llvm-cov --workspace --ignore-run-fail --json --output-path docs/work-packages/20260615-cqr05-erod14-wave2-complexity-001/artifacts/coverage_before.json`
- `cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr05-erod14-wave2-complexity-001/artifacts/lcov_before.info`
- `cargo crap --workspace --lcov docs/work-packages/20260615-cqr05-erod14-wave2-complexity-001/artifacts/lcov_before.info --min 0 --format json --output docs/work-packages/20260615-cqr05-erod14-wave2-complexity-001/artifacts/crap_before.json`

Exit codes:

- `cargo llvm-cov` JSON: `0`
- `cargo llvm-cov` LCOV: `0`
- `cargo crap`: `0`

Target baseline row:

| Function | Line | Cyclomatic | Coverage | CRAP |
| --- | ---: | ---: | ---: | ---: |
| `Wb11HydrologyKernel::run_erod14_wave2` | 6 | 131.0 | 70.14652014652015 | 587.5911363349628 |

Note:

- `cargo crap` reported unmatched LCOV source-file warnings for unrelated
  test-source paths; the target production file was present in the CRAP output.
