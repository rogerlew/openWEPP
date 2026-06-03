# Baseline Provenance Map

Status: completed/HOLD
Evidence mode: static

Static:

- Pinned baseline authority for snowpack state remains `/workdir/wepp-forest_260430_baseline/src/snowd.for` with HPHYS0270 inspection focused on daily carry-state mutation, not a new physics port.
- `snowd.for` lines 61-65 increment/reset `wdayct`; lines 116-140 settle existing snowpack on cold/no-snow days and cap density; lines 180-185 enter warm/melt branch by adding hourly snow/drift to prior depth; lines 193-279 call `melt`, route or retain melt/rain below the `350 kg m^-3` gate, and update depth/density; lines 305-312 publish final daily depth/density state.
- Corrected negative-melt authority remains `/workdir/wepp-forest/src/winter.for` lines 441-460 at commit `03fee4558456535138592630b5dedc4d81ce8d06`, as ratified by HPHYS0269 and `SC-SNOWFREEZE-001#INV-SNOWFREEZE-015`.
- HPHYS0270 identified an observability gap: HPHYS0269 traces exposed post-day snow state and hourly totals but did not explicitly carry day-begin SWE/depth/density/settle-count state and deltas into the classification row.
- No additional baseline-authoritative snow physics defect was proven during this package. Production changes are limited to trace/publication evidence needed to localize the next seam.

Ran:

- Not applicable; this artifact records static provenance inspection.
