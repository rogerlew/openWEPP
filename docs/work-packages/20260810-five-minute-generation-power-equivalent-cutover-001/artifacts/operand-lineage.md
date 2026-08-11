# Operand Lineage

Status: `PASS — isolated publication lineage`

Evidence mode: `Static + Ran`

The optional ledger is produced after `run_r7d6_peak_runoff_span` and before
storage reconciliation. It reads cloned WB14 producer inputs, the unchanged
`wb14_hourly_excess_m` authority, and the R4O hourly saturation-return carry.
The raw replay now publishes rainfall, Green-Ampt infiltration, per-bin
earliest-allocated depression-storage retention, and explicitly named raw
WB14 post-depression generation. It scales raw five-minute generation within
each hour to WB14 authority, adds
`S_h/12` with explicit timing provenance, and stores only a publication
structure on `DirectDayFrame`.

The runner streams that structure through
`HillslopeWatSubhourlyParquetRowGroupWriter`, which independently validates
row domains and raw/hour closure at its public boundary. No water, peak, transfer,
erosion, HBP, or routing state reads it. Power-equivalent rate, duration, and
exponent remain `None`; method is `water_only_no_erosion_adoption`.

Additional supply is not reconstructed: `WAT5-E-001` terminates the requested
diagnostic on positive hourly-only melt/runon. This is an explicit
source-completeness boundary, not a fallback.
