# CQR34 Disposition

Evidence mode: **Static** and **Ran**

## Status

Disposition: complete-with-warnings

## Findings

| Finding | Disposition | Rationale |
| --- | --- | --- |
| Same-file `Wb13DailyWaterBalanceRow::from_surface` remains CRAP `44.95061728395059`. | follow-up | Out of CQR34 target scope; row is behavior-sensitive WB13 output construction and must be handled by a dedicated package. |
| `cargo crap` reports `126 source files had no matching entry in the LCOV report`. | accepted-warning | Recurring workspace source-map warning class observed in previous CQR packages; LCOV and CRAP artifacts were still produced for the target file. |

## Closure Evidence

- [DIRECT] CQR34 target `SummaryAccumulatorError::fmt` final CRAP: `1.0`.
- [DIRECT] Extracted helper `SummaryAccumulatorError::write_display` final
  CRAP: `15.0`.
- [DIRECT] Required gates passed and are recorded in `gate-results.md`.
- [DIRECT] No review finding remains undispositioned.

## Follow-Up

- [DIRECT] First actionable follow-up: consider a future, separately scoped
  CQR package for `Wb13DailyWaterBalanceRow::from_surface` if it appears in a
  ranked live CRAP row.
