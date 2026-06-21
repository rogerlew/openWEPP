# Data-Path Proof

Evidence mode: Static + Ran.

## Pre-Change

R6C state:

- cutover fails before `DirectRunFrame::skeleton`;
- production climate lifecycle does not retain direct publication producers;
- compatibility WB13/runtime/writeback products remain forbidden direct sources.

## Target

`runtime_selection opt-in -> climate lifecycle retained direct producer ->
DirectRunPublicationFrame -> direct artifacts -> fail-closed cutover gates`

## Post-Change

Static:

`DirectPublicationFrameCutover -> build_retained_direct_publication_frame ->
ClimateExecutionAccumulator.retained_direct_publication ->
retain_direct_publication_day_rows -> HillslopeClimateExecution.retained_direct_publication ->
build_direct_publication_artifacts -> validate_retained_direct_publication_frame ->
DirectPublicationExecution -> require_direct_publication_cutover_gates ->
HOLD-R6D-PARITY-GRADE-PUBLICATION-PRODUCERS-ABSENT`

Source authority:

- parsed climate day calendar and precipitation;
- parsed slope-derived per-OFE lane area;
- run/lane/day identity from runner target context.

Forbidden data path:

- no WB13 rows;
- no compatibility runtime publication symbols;
- no writeback payloads;
- no stale logical state;
- no skeleton direct frame for cutover.
