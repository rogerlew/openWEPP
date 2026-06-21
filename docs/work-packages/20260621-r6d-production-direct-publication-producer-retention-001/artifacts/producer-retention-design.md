# Producer Retention Design

Evidence mode: Static + Ran.

## Design

R6D adds a production-retained direct publication frame to the climate-day
lifecycle instead of trying to manufacture direct authority in the output
writer.

Static:

- `execute_hillslope_climate_days` now receives `HillslopeRuntimeSelection`.
- `build_retained_direct_publication_frame` returns `None` for all modes except
  `DirectPublicationFrameCutover`.
- For cutover, the retained frame identity is:
  - `run_id = output_hillslope_id`;
  - `hillslope_id = output_hillslope_id`;
  - `lane_count = per_ofe_lane_areas_m2.len()`;
  - `day_count = climate_span.days.len()`.
- The retained frame metadata uses the actual run name, runtime selection, and
  direct publication cutover output policy.
- `ClimateExecutionAccumulator::retain_direct_publication_day_rows` appends rows
  once per climate day and OFE lane.
- `build_direct_publication_artifacts` consumes
  `execution.retained_direct_publication` for cutover and validates row count
  before building direct projection artifacts.

## Non-Goals Preserved

Static:

- R6D does not activate direct publication by default.
- R6D does not delete compatibility publication writers.
- R6D does not claim HBP/WAT/PASS/loss/manifest parity.
- R6D does not populate missing hydrology/erosion operands from compatibility
  WB13 rows, runtime surfaces, writeback payloads, or stale logical state.
- R6D does not treat the retained frame as sufficient cutover authority.

## Hold

R6D stops at
`HOLD-R6D-PARITY-GRADE-PUBLICATION-PRODUCERS-ABSENT` because the retained frame
currently carries only direct climate/calendar/geometry authority plus
zero/absent placeholders for required output-grade hydrology and erosion
families.
