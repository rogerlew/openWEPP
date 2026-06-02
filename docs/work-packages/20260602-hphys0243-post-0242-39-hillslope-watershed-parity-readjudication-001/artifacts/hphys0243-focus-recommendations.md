# HPHYS0243 Focus Recommendations

Status: complete
Evidence mode: Static + Ran

## Recommendation

Prioritize a contract-first package for coupled WB11 snow/storage/ET and WB18
early-transient overdrainage lineage before doing more WB14/WB12 runoff
handoff work.

## Rationale

1. `Q`/`QOFE` are semantically closed in the fresh HPHYS0243 run, so additional
   runoff publication work is not the highest-yield next step.
2. `Total-Soil` and `SoilWaterTotal` remain the dominant fail-saturated
   residuals (`39/39`, `140.707505 mm` mean absolute difference).
3. `Snow-Water` is now a first-order residual (`39/39`, `91.221051 mm` mean
   absolute difference), and signed probes show candidate snow storage is below
   baseline.
4. `Ep` is consistently under baseline while `Es` is over baseline, indicating
   ET partition/stage-memory lineage remains coupled to storage closure.
5. First-week `Dp` remains materially over baseline on representative
   hillslopes even though long-run average `Dp` is sub-millimeter; this points
   to mutable state initialization/update order, not merely daily publication.

## Proposed Next Package

`20260602-hphys0244-wb11-snow-storage-et-dp-coupled-lineage-diagnostics-001`

Scope:
- No production edits initially.
- Compare baseline and openWEPP day-1..30 trajectories for `H1`, `H7`, and
  `H39` across:
  - `Snow-Water`, `RM`, `P`
  - `Ep`, `Es`, `Er`, `ET`
  - `Dp`, `Pe`, layer `theta/st`, `Total-Soil`, `SoilWaterTotal`
  - WB18/WB11 mutable storage seed and update surfaces.
- Trace baseline `watbal_hourly`, `snow`, `evap/evappm`, and `purk/perc`
  order against openWEPP runtime state publications.
- Publish one implementation-ready queue item that distinguishes:
  - snow/rain partition and `snow.runtime_swe` mutation,
  - ET partition/stage-memory lineage,
  - WB18 early transient overdrainage/state mutation,
  - profile FC storage authority residual.

## Separate Watershed Follow-Up

If watershed parity is the immediate priority, create a separate watershed
output-span package. The fresh watershed run passed, but semantic comparison is
blocked by shape: current candidate interchange emits one-row surfaces for
daily-span baseline files such as `ebe_pw0`, `chan.out`, `chanwb`,
`soil_pw0`, and `totalwatsed3`.
