# Layer Retention Localization Evidence

Status: complete
Evidence mode: Static + Ran

## Evidence

Static:
- Baseline `watbal_hourly.for` applies post-ET lower-layer upper-limit redistribution before drainage/lateral/root uptake publication.
- The implemented seam conserves aggregate profile storage while moving lower-layer excess upward, so immediate `Total-Soil` change is indirect through later drainage/lateral/root uptake.

Ran:
- Rebuilt-trace root: `/tmp/hphys0286_trace_rebuilt_20260604T212019Z`
- H7 trace over 2014 Julian 120-170:
  - Lower-layer over-UL rows after rebuilt seam: `19`, all immediately after `percolation_deep_seepage`.
  - No lower-layer over-UL rows persist in the sampled `evapotranspiration`, `plant_root_uptake`, or `storage_reconciliation` phase rows.
- H39 trace over 2014 Julian 120-170:
  - Lower-layer over-UL rows: `0`.

Static + Ran:
- The WB17 seam is active and reduces downstream lateral/ET/storage residuals.
- Remaining spring storage collapse is driven by upstream snow/runoff/infiltration water availability and/or percolation behavior before the ET seam, not by persistent post-ET lower-layer overcapacity.
