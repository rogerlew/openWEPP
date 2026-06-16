# CQR34 Coverage Closure

Evidence mode: **Ran**

## Target-File LCOV Summary

| Snapshot | FNF | FNH | Function Coverage | LF | LH | Line Coverage |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| Before | 59 | 53 | 89.830508474576% | 745 | 646 | 86.711409395973% |
| After | 63 | 60 | 95.238095238095% | 877 | 826 | 94.184720638540% |

## Interpretation

- [DIRECT] Target-file line coverage increased from
  `86.711409395973%` to `94.184720638540%`.
- [DIRECT] Target-file function coverage increased from
  `89.830508474576%` to `95.238095238095%`.
- [DIRECT] The added coverage is focused on
  `SummaryAccumulatorError::fmt`, `SummaryAccumulatorError::write_display`,
  `SummaryAccumulatorError::source`, and `SummaryWindow::as_str` display use.
- [DIRECT] Coverage was generated from full workspace `cargo llvm-cov` runs,
  not from a crate-only shortcut.
