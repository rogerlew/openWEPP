# CQR08 Quality Plan Report

Static: the scoped quality target was localized function-length and lint-debt
burndown for `HillslopeRuntimeInputError::fmt` and the high-complexity adjacent
`code()` matcher in `00_core_types.rs`.

Static: protected boundaries were public API, error variants, error fields,
stable `HS-RUNTIME-E-*` codes, display text, guard thresholds, symbols, aliases,
runtime projection behavior, and process-physics math.

Ran: baseline focused runtime-input tests passed before production refactor.

Ran: all-variant characterization was added before production decomposition and
passed before and after the production refactor.

Ran: closure metrics show:

| Metric | Before | After |
| --- | ---: | ---: |
| Target-file LCOV lines hit / found | 24 / 425 | 497 / 515 |
| Target-file LCOV functions hit / found | 1 / 2 | 20 / 20 |
| `HillslopeRuntimeInputError::code` CRAP | 964.0467577461321 | 9.0 |
| `HillslopeRuntimeInputError::fmt` CRAP | 4290.0 | 9.0 |
| Maximum target-file CRAP row after refactor | 4290.0 | 14.0478515625 |

Disposition: quality target closed.
