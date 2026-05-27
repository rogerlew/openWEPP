# WSHEDIMPL06 Disposition

Status: package-complete-with-hold
Evidence mode: static+ran
Date: 2026-05-27
Decision: HOLD

## Static
- WSHED06 objective is complete for scoped WS11 channel sediment
  publication-family closure:
  - WS10 channel nodes publish `ws10_channel_{id}_qsed` and
    `ws10_channel_{id}_tc`.
  - WSHED03 WS11 sediment publication vector is active and passing.
- Contract gap posture is narrowed:
  - `GAP-ROUTE-009` now tracks remaining full
    `chnero/chnrt/detach` process-parity migration and downstream validation,
    not publication-family symbol absence.
  - Companion dependency rows `GAP-SED-006` and `GAP-SYSTEM-008` reflect
    WSHED06 publication-family closure while preserving non-promotable full
    process integration scope.
- Residual system blockers keep program-level watershed closure in `HOLD`:
  - remaining full watershed channel sediment process migration (`chnero`,
    `chnrt`, `detach`),
  - `GAP-IMPOUND-005` (WSHED07 impoundment regime-transition parity),
  - `GAP-SYSTEM-006` (WSHED08 watershed parquet writer activation),
  - WSHED09 end-to-end validation/disposition still pending.

## Ran
- Scoped validation and gate commands were executed (see `gate-results.md` and
  `wshedimpl06-implementation-and-test-evidence.md`).
