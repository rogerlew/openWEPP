# Implementation

Status: `EXECUTED-HOLD-GWDSV-CHANNEL-CONSUMER`

Evidence: `Static + ran`

## Production Changes

- Added `DirectGroundwaterAuthority`, `DirectGroundwaterRunState`, and
  `DirectGroundwaterDayOutput` in
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime/groundwater.rs`.
- Added run-level groundwater state to `DirectRunFrame`, initialized disabled
  by default and configured from parsed `gwcoeff.txt` authority before direct
  publication streaming.
- Implemented the contract recurrence:
  `S_i = S_(i-1) + D_i - Qb_(i-1) - Qs_(i-1)`,
  `Qb_i = bfcoeff * S_i`, and `Qs_i = dscoeff * S_i`.
- Aggregated Lane D recharge after all lane hydrology has executed:
  `D_i = sum(deep_percolation_m * lane.area_m2)`.
- Recorded generated groundwater totals in `DirectLanedActiveRunSummary` without
  feeding generated baseflow or reservoir deep seepage into
  `laned_active_lane_source`.
- Parsed `gwcoeff.txt` through the existing `SC-INFILE-GWCOEFF-001` parser in
  both legacy sidecar discovery and runfile sidecar modes. Missing
  `gwcoeff.txt` remains disabled (`lr_bf=0`); present malformed parser errors
  fail closed.
- Added direct WAT nullable `Base` column and unit registry authority
  (`hillslope_wat.Base:mm`) for generated groundwater baseflow.
- Mapped direct publication `groundwater_baseflow_mm` to WAT `Base`; the
  existing watershed WAT consumer reads `Base` into `baseflow_mm` and
  `channel_baseflow_m3`.

## Tests Added Or Updated

- `gwbaseflow_linear_reservoir_recurrence_uses_prior_day_exports`
- `gwbaseflow_mofe_recharge_aggregates_lane_deep_percolation`
- `gwbaseflow_exports_over_accepted_storage_fail_closed`
- `r6a_direct_projection_consumers_read_publication_frame_operands` now proves
  direct publication groundwater baseflow maps to WAT `Base`.
- WAT writer/schema fixtures and integration WAT fixtures now include nullable
  `Base`.

## Hold Boundary

Generated baseflow (`gwbfv`) has real downstream consumer proof through direct
WAT `Base` and `crates/openwepp-runner/src/watershed_wat.rs`.

Generated groundwater-reservoir deep seepage (`gwdsv`) is computed and recorded
in the direct runtime and active summary, but no real downstream watershed/HBP
consumer was moved in-envelope in this package. `bftharea` is parsed and carried
as authority, but the watershed/channel threshold branch is not implemented in
the current Lane D hillslope package.

Closure therefore exits as
`EXECUTED-HOLD-GWDSV-CHANNEL-CONSUMER`, not full M-T2B closure.

## Line-Count Governance

The bounded direct-frame layout test was updated from `DirectDayFrame <= 15_488`
to `<= 15_536`, with a package-local comment explaining the expected 48-byte
increase from carrying `DirectGroundwaterDayOutput`.
