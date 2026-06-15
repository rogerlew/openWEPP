# CRAP Before

Ran: before metrics were captured with:

```sh
cargo llvm-cov clean --workspace
cargo llvm-cov --workspace --ignore-run-fail --lcov \
  --output-path docs/work-packages/20260615-cqr08-runtime-core-types-display-001/artifacts/lcov_before.info
cargo crap --workspace \
  --lcov docs/work-packages/20260615-cqr08-runtime-core-types-display-001/artifacts/lcov_before.info \
  --min 0 --format json \
  --output docs/work-packages/20260615-cqr08-runtime-core-types-display-001/artifacts/crap_before.json
```

Ran: target rows before refactor:

| Function | Line | Cyclomatic | Coverage | CRAP |
| --- | ---: | ---: | ---: | ---: |
| `HillslopeRuntimeInputError::code` | 319 | 65.0 | 40.298507462686565 | 964.0467577461321 |
| `HillslopeRuntimeInputError::fmt` | 391 | 65.0 | 0.0 | 4290.0 |

Disposition: both target rows exceeded the ADR-0021 package threshold.
