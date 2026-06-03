# Worker Handoff

Status: completed

Evidence mode: static+ran

## Continuation Recommendation

Scaffold HPHYS0262 for baseline-authoritative WB17 `evap` demand seeding and
plant-state initialization/call-order lineage.

## Recommended Objective

Diagnose and correct, if proven by baseline authority, why H1/H7/H39 day-1
candidate `Etp/Ep` is `0.385294 mm` while baseline WAT `Ep` is `0.150000 mm`.
The next package should focus on the state consumed by `evap.for` before daily
`ptgrp`/`ptgra`, not on SWU stress clipping.

## Required Evidence

- Compare openWEPP pre-`evap` and post-growth `lai`, `cancov`, `eo`/`Eu`,
  `Etp`, and seeded pre-`swu` `Ep` against
  `/workdir/wepp-forest_260430_baseline`.
- Preserve the legacy ordering distinction:
  `evap` seeds demand before daily growth/root update; `swu` consumes that
  demand after growth/root update.
- Add trace fields for the pre-`evap` plant/ET seed row if current trace
  boundaries cannot distinguish pre-growth and post-growth state.
- Rerun H1/H7/H39 and full H1..H39 semantic metrics.

## Carry-Forward Metrics

Ran: HPHYS0261 full-suite metrics from
`/tmp/hphys0261_20260603T042648Z`:

- Semantic pass: `0/39`
- `Ep` mean abs diff mean: `1.689334`; max abs diff: `7.779383`
- `Total-Soil`/`SoilWaterTotal` mean abs diff mean: `152.388768`; max abs
  diff: `616.171444`
- `Dp` mean abs diff mean: `0.151072`; max abs diff: `0.244800`
- `latqcc` mean abs diff mean: `0.675393`; max abs diff: `14.760000`
- `Q` mean abs diff mean: `0.925027`; max abs diff: `194.715728`
- `RM` mean abs diff mean: `2.301802`; max abs diff: `204.850510`
- `Snow-Water` mean abs diff mean: `58.195696`; max abs diff: `562.470000`
