# CQR34 CRAP Before

Evidence mode: **Ran**

## Commands

```text
cargo llvm-cov clean --workspace
cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr34-summary-accumulator-complexity-001/artifacts/lcov_before.info
cargo crap --workspace --lcov docs/work-packages/20260615-cqr34-summary-accumulator-complexity-001/artifacts/lcov_before.info --min 0 --format json --output docs/work-packages/20260615-cqr34-summary-accumulator-complexity-001/artifacts/crap_before.json
```

## Result

- [DIRECT] `cargo llvm-cov` completed successfully and wrote
  `lcov_before.info`.
- [DIRECT] `cargo crap` completed successfully and wrote `crap_before.json`.
- [DIRECT] `cargo crap` reported the recurring workspace LCOV source-map
  warning class: `126 source files had no matching entry in the LCOV report`.

## Target Rows

| Function | Line | CC | Coverage | CRAP |
| --- | ---: | ---: | ---: | ---: |
| `SummaryAccumulatorError::fmt` | 738 | 15.0 | 0.0% | 240.0 |
| `SummaryAccumulatorError::source` | 795 | 4.0 | 0.0% | 20.0 |
| `SummaryWindow::as_str` | 69 | 5.0 | 0.0% | 30.0 |
| `Wb13DailyWaterBalanceRow::from_surface` | 228 | 33.0 | 77.77777777777779% | 44.95061728395059 |

## Disposition

- [DIRECT] `SummaryAccumulatorError::fmt` is the CQR34 live target.
- [DIRECT] `Wb13DailyWaterBalanceRow::from_surface` is a same-file
  out-of-scope row above CRAP `30`; it is recorded as a warning and not changed
  by this display-target package.
