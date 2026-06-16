# CQR34 Quality Plan Report

Evidence mode: **Static** and **Ran**

## Target

- [DIRECT] ExecPlan row: CQR34, rank 28, original CRAP `240`, CC `15`,
  coverage `0%`.
- [DIRECT] Target file:
  `crates/openwepp-summary-accumulator/src/lib.rs`.
- [DIRECT] Live before target from `crap_before.json`:
  `SummaryAccumulatorError::fmt`, line `738`, CC `15.0`, coverage `0.0%`,
  CRAP `240.0`.

## Scope

- [DIRECT] Package scope is private, behavior-preserving decomposition of the
  high-CRAP display target plus focused characterization tests.
- [DIRECT] Protected surfaces: public accumulator APIs, summary window
  transitions, WB13 output symbols, output formulas, float expression order,
  status message IDs, comparator metadata routing, typed error variants,
  source behavior, parser compatibility, and science-contract behavior.
- [DIRECT] Same-file out-of-scope high-CRAP row remains
  `Wb13DailyWaterBalanceRow::from_surface` at CRAP
  `44.95061728395059`.

## Plan

1. Capture before line counts, suppression census, LCOV, and CRAP.
2. Add characterization tests for all `SummaryAccumulatorError` display arms
   and `source()` behavior before production refactor.
3. Extract a private `SummaryAccumulatorError::write_display` helper and keep
   the original match arms/message strings unchanged.
4. Capture after LCOV and CRAP.
5. Run required gates, complete dual review and verification, then commit and
   push before updating the ExecPlan tracker.

## Risk Controls

- [DIRECT] No public symbol, enum variant, output symbol, status message ID,
  comparator routing, or formula is changed.
- [DIRECT] No fallback/default wrapper is added.
- [DIRECT] No dependency, unsafe, serialization, parser, or runtime-publication
  behavior is changed.
- [DIRECT] The focused tests assert exact display strings for all variants and
  source preservation for wrapped errors.
