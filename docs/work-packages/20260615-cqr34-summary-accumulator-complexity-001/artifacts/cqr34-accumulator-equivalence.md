# CQR34 Accumulator Equivalence

Evidence mode: **Static** and **Ran**

## Static Equivalence Review

- [DIRECT] `SummaryAccumulatorError::fmt` now delegates to private
  `SummaryAccumulatorError::write_display`.
- [DIRECT] `write_display` contains the same match arms and display strings
  as the previous `fmt` implementation.
- [DIRECT] `SummaryAccumulatorError` variants are unchanged.
- [DIRECT] `impl Error for SummaryAccumulatorError` remains public-behavior
  equivalent; the existing `source()` match was not structurally changed.
- [DIRECT] No accumulator state machine, rollup window, WB13 output symbol,
  output formula, float expression, status message ID, comparator metadata
  routing, or parser/runtime-publication path was changed.

## Characterization Coverage

- [DIRECT] Added exact display-string tests for validation/window variants:
  `InvalidDate`, `EmptyScalarSurface`, `EmptySymbol`, `DuplicateSymbol`,
  `NonFiniteInput`, `NonMonotonicDate`, `WindowStateMissing`,
  `WindowTotalsMissing`, and `FinalizeWithoutSamples`.
- [DIRECT] Added exact display-string tests for WB13/output/wrapped variants:
  `MissingRequiredOutputSymbol`, `OutputSymbolOutOfRange`,
  `NonMonotonicOutputRow`, `Status`, and `ComparatorMetadata`.
- [DIRECT] Added `source()` characterization for wrapped status and comparator
  errors, plus a non-wrapped no-source branch.

## Ran Evidence

```text
cargo test -p openwepp-summary-accumulator
```

- [DIRECT] Before production refactor: passed, `13` tests.
- [DIRECT] After production refactor: passed, `13` tests.
- [DIRECT] Full after LCOV workspace run also passed the
  `openwepp_summary_accumulator` unit-test suite with `13` tests.
