# PERFIDX03 Verification B

Status: HOLD 2026-06-17
Evidence mode: **Static** + **Ran**

Verification focus: final-tree safety and non-deferral.

## Verified

- `HillslopeClimateExecutionState` carries the run-scoped registry, so missing
  registry construction is surfaced as a runtime-surface failure instead of a
  silent default.
- `OfeLanePersistentState` contains optional indexed authority state and can
  clone/export from it, but the runner does not activate it in final setup.
- `OfeLanePersistentStateSequence::refresh_indexed_writeback_authority` refreshes
  only lanes that were already indexed-active. It no longer flips authority on
  after the first day.
- The final OFE5 no-flip run produced strict output hash identity for `H1.hbp`,
  `H1.loss.json`, `H1.wat.parquet`, and `H1.plot.parquet`; `H1.pass.parquet`
  logical rows compare equal by DuckDB.

## Verification Result

The final tree avoids the measured hot-path regression. PERFIDX03 remains held
because the package's current-scope authority-flip and full-anchor gates are not
complete.
