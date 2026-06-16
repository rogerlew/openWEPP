# CQR34 CRAP After

Evidence mode: **Ran**

## Commands

```text
cargo llvm-cov clean --workspace
cargo llvm-cov --workspace --ignore-run-fail --lcov --output-path docs/work-packages/20260615-cqr34-summary-accumulator-complexity-001/artifacts/lcov_after.info
cargo crap --workspace --lcov docs/work-packages/20260615-cqr34-summary-accumulator-complexity-001/artifacts/lcov_after.info --min 0 --format json --output docs/work-packages/20260615-cqr34-summary-accumulator-complexity-001/artifacts/crap_after.json
```

## Result

- [DIRECT] `cargo llvm-cov` completed successfully and wrote
  `lcov_after.info`.
- [DIRECT] `cargo crap` completed successfully and wrote `crap_after.json`.
- [DIRECT] `cargo crap` reported the recurring workspace LCOV source-map
  warning class: `126 source files had no matching entry in the LCOV report`.

## Target Rows

| Function | Line | CC | Coverage | CRAP |
| --- | ---: | ---: | ---: | ---: |
| `SummaryAccumulatorError::fmt` | 738 | 1.0 | 100.0% | 1.0 |
| `SummaryAccumulatorError::write_display` | 744 | 15.0 | 100.0% | 15.0 |
| `SummaryAccumulatorError::source` | 801 | 4.0 | 100.0% | 4.0 |
| `SummaryWindow::as_str` | 69 | 5.0 | 71.42857142857143% | 5.5830903790087465 |
| `Wb13DailyWaterBalanceRow::from_surface` | 228 | 33.0 | 77.77777777777779% | 44.95061728395059 |

## Closure

- [DIRECT] CQR34 target `SummaryAccumulatorError::fmt` is CRAP `1.0`.
- [DIRECT] Newly extracted private helper
  `SummaryAccumulatorError::write_display` is CRAP `15.0`.
- [DIRECT] `SummaryAccumulatorError::source` improved to CRAP `4.0` through
  characterization coverage.
- [DIRECT] Same-file out-of-scope row
  `Wb13DailyWaterBalanceRow::from_surface` remains CRAP
  `44.95061728395059` and is unchanged by this package.
