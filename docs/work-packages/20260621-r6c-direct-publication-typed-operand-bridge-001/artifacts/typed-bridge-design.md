# Typed Bridge Design

Evidence mode: Static.

R6C bridge requirements:

- bridge starts from parsed inputs and accepted direct run/lane/day operands;
- missing required direct operands produce typed fail-closed errors;
- skeleton/zero direct rows cannot satisfy cutover gates;
- compatibility WB13 rows, runtime surfaces, writeback payloads, stale logical
  state, and wrappers around them are forbidden as accepted direct authority;
- bridge populates all direct projection operands required for HBP, WAT, PASS,
  loss, and manifest.

Implementation notes will be appended during execution.

## Execution Notes

Static:

- `execute_hillslope_climate_days` publishes WB13/PASS compatibility rows from
  `DailyExecutionResult` and `PersistentDailyExecutionResult`.
- Those result types retain `runtime_surface`, `wb13_row` or
  `internal_wb13_collection`, PASS rows, coupling vectors, and scheduler
  provenance.
- They do not retain `DirectDayFrame`, `DirectRunPublicationFrame`, or accepted
  direct publication operands from the production scheduler lifecycle.
- The R6B candidate therefore created an independent skeleton direct frame after
  compatibility execution.

Code change:

- `build_direct_publication_artifacts` now refuses
  `DirectPublicationFrameCutover` before constructing a skeleton frame.
- The fail-closed marker is
  `HOLD-R6C-DIRECT-PHASE-PUBLICATION-PRODUCER-ABSENT`.

Required next design:

- add a retained direct publication producer surface to the production climate
  lifecycle;
- populate that surface from direct run/lane/day state while production direct
  phases execute;
- only then build HBP/WAT/PASS/loss/manifest projections and parity gates.
