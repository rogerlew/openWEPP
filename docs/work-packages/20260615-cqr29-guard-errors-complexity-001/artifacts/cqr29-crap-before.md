# CQR29 CRAP Before

Ran:

```text
cargo llvm-cov clean --workspace
cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr29-guard-errors-complexity-001/artifacts/lcov_before.info
cargo crap --workspace --lcov docs/work-packages/20260615-cqr29-guard-errors-complexity-001/artifacts/lcov_before.info --min 0 --format json --output docs/work-packages/20260615-cqr29-guard-errors-complexity-001/artifacts/crap_before.json
```

Baseline target-file rows:

| Function | Line | CC | Coverage | CRAP |
| --- | ---: | ---: | ---: | ---: |
| `Wb11HydrologyKernelGuardError::fmt` | 167 | 16 | 0.0 | 272.0 |
| `Wb11HydrologyKernelGuardError::code` | 100 | 24 | 90.625 | 24.474609375 |
| `Wb11HydrologyKernelGuardError::boundary_class` | 79 | 4 | 100.0 | 4.0 |

LCOV target-file summary: `FNF:3`, `FNH:2`, `LF:139`, `LH:35`.

Warn: `cargo crap` emitted 126 source-map warnings for source files with no
matching LCOV report entry.
