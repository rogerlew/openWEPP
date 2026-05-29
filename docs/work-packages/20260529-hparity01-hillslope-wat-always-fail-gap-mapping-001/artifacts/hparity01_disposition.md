# HPARITY01 Disposition

Status: completed  
Evidence mode: Static + Ran

## Decision
- `HOLD`

## Completed in HPARITY01
1. Built full 12-column always-fail gap matrix with:
   - contract anchors,
   - baseline residual fingerprints,
   - runtime writer ownership,
   - guard-family mapping,
   - follow-on wave ownership.
2. Added canonical contract disambiguation/lineage amendments:
   - HPARITY01 WB13 lineage register in `SC-WATBAL-001`,
   - `Dp` disambiguation in `SC-PERC-001` and `SC-CLIMATE-001`,
   - index metadata updates in `science-contracts/index.md`.
3. Added contract-derived integration test scaffold:
   `hparity01_hillslope_wat_lineage_contract.rs`.

## Why HOLD Remains
- HPARITY01 is scaffolding only by design and explicitly excludes production
  closure for the 12 failing columns.
- Non-zero residuals remain across all 39 hillslopes for all 12 columns in the
  baseline evidence snapshot.

## Closure Wave Ownership
1. `HPARITY02`: profile storage/capacity family
   (`ProfileDepth`, `ProfilePorosityCap`, `ProfileFCStore`, `ProfileWPStore`).
2. `HPARITY03`: ET + rain/snow publication family
   (`Ep`, `Es`, `RM`, `Snow-Water`).
3. `HPARITY04`: percolation/subsurface/aggregate family
   (`Dp`, `latqcc`, `Total-Soil`, `SoilWaterTotal`).
4. `HPARITY05`: end-to-end rerun and full hold-lift closeout.
