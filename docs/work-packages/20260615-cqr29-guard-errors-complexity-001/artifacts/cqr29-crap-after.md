# CQR29 CRAP After

Ran:

```text
cargo llvm-cov clean --workspace
cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr29-guard-errors-complexity-001/artifacts/lcov_after.info
cargo crap --workspace --lcov docs/work-packages/20260615-cqr29-guard-errors-complexity-001/artifacts/lcov_after.info --min 0 --format json --output docs/work-packages/20260615-cqr29-guard-errors-complexity-001/artifacts/crap_after.json
```

Final target-file rows:

| Function | Line | CC | Coverage | CRAP |
| --- | ---: | ---: | ---: | ---: |
| `Wb11HydrologyKernelGuardError::code` | 100 | 24 | 93.75 | 24.140625 |
| `Wb11HydrologyKernelGuardError::phase_display_parts` | 184 | 8 | 97.72727272727273 | 8.000751314800901 |
| `HydrologyGuardErrorDisplayParts::fmt_with_code` | 330 | 7 | 100.0 | 7.0 |
| `Wb11HydrologyKernelGuardError::erod13_display_parts` | 236 | 5 | 94.73684210526315 | 5.003644846187491 |
| `Wb11HydrologyKernelGuardError::erod14_display_parts` | 260 | 5 | 94.73684210526315 | 5.003644846187491 |
| `Wb11HydrologyKernelGuardError::erod18_display_parts` | 284 | 5 | 95.65217391304348 | 5.00205473822635 |
| `Wb11HydrologyKernelGuardError::display_parts` | 164 | 5 | 100.0 | 5.0 |
| `Wb11HydrologyKernelGuardError::boundary_class` | 79 | 4 | 100.0 | 4.0 |
| `Wb11HydrologyKernelGuardError::fmt` | 378 | 1 | 100.0 | 1.0 |

LCOV target-file summary: `FNF:9`, `FNH:9`, `LF:191`, `LH:184`.

Warn: `cargo crap` emitted 126 source-map warnings for source files with no
matching LCOV report entry.
