# Disposition

Status: complete
Evidence mode: Static + Ran

## Static: Package Result

- HPHYS0284 is complete for its declared scope.
- The localized defect was corrected negative-melt carry-state lineage: routed melt and snowpack state loss were incorrectly collapsed to the same net-melt quantity.
- Canonical authority now lives in `SC-SNOWFREEZE-001#INV-SNOWFREEZE-019` and `SC-WATBAL-001#INV-WATBAL-059`.
- Production code now separates routed melt from carried snowpack state loss and fails closed on non-finite/materially negative post-loss runtime SWE.

## Ran: Validation Summary

- Contract-derived red gate failed before production edits and passed after production edits.
- Focused snow runtime and adjacent hydrology tests passed.
- Full Rust gates passed after formatting.
- Full H1..H39 suite completed all 39 hillslopes and all 39 semantic reports.

## Ran: Semantic Result

- HPHYS0284 improved `Snow-Water` mean abs diff from `4.909469` to `2.899431`.
- HPHYS0284 improved `RM` mean abs diff from `0.324492` to `0.248018`.
- HPHYS0284 improved `Q` mean abs diff from `0.672385` to `0.552218`.
- `Total-Soil`, `SoilWaterTotal`, `Ep`, and `Dp` worsened modestly, so full semantic closure remains open.

## Static: Continuation

- Next package should focus spring runoff/infiltration/storage partition after snowmelt timing, not WB17 `Ep` compensation.
- Priority evidence window remains H1/H7/H39 Julian 120-147, with emphasis on same-day liquid partition, runoff carry-through, layer storage mutation, and aggregate storage recomputation after corrected snow meltout.
