# CRAP After

Ran: after metrics were captured with:

```sh
cargo llvm-cov clean --workspace
cargo llvm-cov --workspace --ignore-run-fail --lcov \
  --output-path docs/work-packages/20260615-cqr08-runtime-core-types-display-001/artifacts/lcov_after.info
cargo crap --workspace \
  --lcov docs/work-packages/20260615-cqr08-runtime-core-types-display-001/artifacts/lcov_after.info \
  --min 0 --format json \
  --output docs/work-packages/20260615-cqr08-runtime-core-types-display-001/artifacts/crap_after.json
```

Ran: maximum target-file CRAP after refactor is `14.0478515625`
(`HillslopeRuntimeInputError::soil_core_code`).

Ran: selected target rows after refactor:

| Function | Line | Cyclomatic | Coverage | CRAP |
| --- | ---: | ---: | ---: | ---: |
| `HillslopeRuntimeInputError::code` | 319 | 9.0 | 100.0 | 9.0 |
| `HillslopeRuntimeInputError::fmt` | 1183 | 9.0 | 100.0 | 9.0 |
| `HillslopeRuntimeInputError::soil_core_code` | 388 | 14.0 | 93.75 | 14.0478515625 |
| `HillslopeRuntimeInputError::fmt_soil_core` | 502 | 14.0 | 97.72727272727273 | 14.002300901577762 |
| `HillslopeRuntimeInputError::pl_projection_code` | 470 | 12.0 | 92.85714285714286 | 12.052478134110787 |

Ran: test helper rows in `08_tests/core_types.rs` all have CRAP `1.0`.

Disposition: scoped CRAP target closed.
