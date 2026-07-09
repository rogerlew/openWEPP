# Source Map

Status: `EXECUTED-HOLD-GWDSV-CHANNEL-CONSUMER`

Evidence: `Static + ran`

Pre-implementation source map for the first implementation pass.

| Source | Consumer / destination | Binding decision |
|---|---|---|
| `crates/openwepp-input-contract/src/parsers/gwcoeff.rs` `GwcoeffFile` | Runner `HillslopeSidecarResolution` groundwater authority | Parse `gwcoeff.txt` as an optional sidecar. Missing emits disabled authority; present malformed/out-of-domain parser errors fail closed. |
| `DirectDayFrame.hydrology_projection.deep_percolation_m` | Groundwater recharge `D_i` | Aggregate Lane D day recharge as `sum(lane.deep_percolation_m * lane.area_m2)` after every lane's hydrology half has executed. This is the soil deep-percolation producer, not generated reservoir deep seepage. |
| Run-level groundwater carry | `S_i`, prior `Qb`, prior `Qs` | Store per-hillslope groundwater carry on `DirectRunFrame`; initialize `S_0 = igwstrd_mm / 1000 * sum(lane.area_m2)` when authority is enabled. |
| Srivastava recurrence output | `gwbfv` / `gwdsv` export state | Compute daily generated baseflow and groundwater-reservoir deep seepage as hillslope-day volumes in `m^3`; assign generated baseflow publication depth only to the terminal row using terminal-row area so watershed aggregation reconstructs the exact generated volume. |
| `laned_active_lane_source` | Active surface-router source series | No groundwater fields are read by the active source builder; active source remains `wb14_hourly_excess + ui_SCrunf + routed melt` weighted to `q_runoff_m`. |
| `DirectLanedActiveRunSummary` | Active ledger/export totals | Add generated groundwater baseflow and generated groundwater-reservoir deep seepage totals next to existing routed surface, clamp, mesh storage, and `latqcc` totals. |
| Direct WAT parquet `Base` | `crates/openwepp-runner/src/watershed_wat.rs` WAT consumer | Populate generated groundwater baseflow depth in `Base`; existing watershed WAT aggregator reads it into `baseflow_mm` and `channel_baseflow_m3`. |
| Generated `gwdsv` | Hold boundary | Computed and recorded in direct runtime/active summary, but no real downstream HBP/pass/watershed consumer was implemented in this package. |
