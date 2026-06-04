# Contract Implementation Evidence

Status: complete
Evidence mode: Static

## Evidence

Static:
- `SC-PERC-001` now records baseline hourly post-ET upper-limit redistribution authority from `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for:564-590`.
- `SC-PERC-001` adds `INV-PERC-018` for lower-layer post-ET excess movement upward with frozen-adjusted caps when same-pass outside water is active.
- `SC-EVAP-001` now records the same WB17 ET seam as `INV-EVAP-026`, including the ET-phase requirement to apply redistribution before aggregate storage writeback.
- `SC-WATBAL-001` now records `INV-WATBAL-061` so daily storage publication consumes redistributed layer state rather than stale post-percolation layer storage.
- Contract versions were incremented: `SC-PERC-001` to `27`, `SC-EVAP-001` to `25`, and `SC-WATBAL-001` to `105`.
