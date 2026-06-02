# HPHYS0244 Prior Root-Cause Review

Static: prior HPHYS artifact review.
Ran: targeted search across HPHYS0211, HPHYS0212, HPHYS0213, HPHYS0235,
HPHYS0236, and HPHYS0243 artifact sets.

## Prior Evidence Chain
1. `HPHYS0211` already classified `Dp`, `Total-Soil`, and `SoilWaterTotal` as
   coupled lifecycle/storage residuals:
   - `Dp` failed all `1461/1461` rows for all `39` hillslopes.
   - `Total-Soil` and `SoilWaterTotal` failed all hillslopes with identical
     per-hillslope ranges.
   - `HP211-RC-001` attributed `Dp` to WB11/WB18 reseeding and WB18 layer
     state consumed by percolation before WB13 publishes `D` as `Dp`.
   - `HP211-RC-004` attributed `Total-Soil` and `SoilWaterTotal` to
     `wb11_soil_water` aggregate lineage and stale/misaligned mutation risk.
2. `HPHYS0212` landed a no-daily-reseed regression guard, but its disposition
   still recorded saturated `Dp`, `Total-Soil`, and `SoilWaterTotal` lanes.
3. `HPHYS0213` added WB19 aggregate-update behavior for `wb11_soil_water`, but
   verification still recorded `39/39` failures for `Dp`, `Total-Soil`, and
   `SoilWaterTotal`.
4. `HPHYS0235` isolated the `H1` early-transient `Dp` mismatch to the hourly
   lane: day-1..7 hourly `Dp` was `~7.26x` baseline while the daily lane was
   near baseline.
5. `HPHYS0236` implemented hourly iterative WB18 execution, but its residual
   matrix recorded worse `Dp`, `Total-Soil`, and `SoilWaterTotal` means than
   the prior HPHYS0234 evidence.
6. `HPHYS0243` fresh post-HPHYS0242 readjudication left the same storage family
   dominant:
   - `Total-Soil`: `39/39`, `140.707505 mm` mean absolute difference.
   - `SoilWaterTotal`: `39/39`, `140.707505 mm` mean absolute difference.
   - `Dp`: `39/39`, `0.288527 mm` mean absolute difference, with large
     first-week overdrain on representative hillslopes.

## Key Prior Artifact References
- `docs/work-packages/20260530-hphys0211-coupled-threshold-root-cause-ledger-001/artifacts/hphys0211-residual-gap-matrix.md`
- `docs/work-packages/20260530-hphys0212-wb11-wb19-lifecycle-coupling-closure-001/artifacts/hphys0212-residual-gap-matrix.md`
- `docs/work-packages/20260530-hphys0213-wb12-storage-and-aggregate-reconciliation-closure-001/artifacts/hphys0213-residual-gap-matrix.md`
- `docs/work-packages/20260601-hphys0235-wb18-dp-7x-legacy-root-cause-closure-001/artifacts/hphys0235-residual-authority-gap-matrix.md`
- `docs/work-packages/20260601-hphys0236-wb18-hourly-iterative-execution-closure-001/artifacts/hphys0236-residual-authority-gap-matrix.md`
- `docs/work-packages/20260602-hphys0243-post-0242-39-hillslope-watershed-parity-readjudication-001/artifacts/hphys0243-hillslope-semantic-summary.md`
- `docs/work-packages/20260602-hphys0243-post-0242-39-hillslope-watershed-parity-readjudication-001/artifacts/hphys0243-focus-recommendations.md`

Captured excerpts:
- `/tmp/hphys0244_20260602T045926Z/hphys0211_key_excerpt.txt`
- `/tmp/hphys0244_20260602T045926Z/hphys0235_key_excerpt.txt`
- `/tmp/hphys0244_20260602T045926Z/hphys0243_focus_excerpt.txt`

## Finding
The HPHYS history does not support treating `Total-Soil`/`SoilWaterTotal`,
`Dp`, and WB18 `Pe` as independent bugs. The recurring evidence supports one
coupled focus area: WB11 aggregate storage continuity and WB18 mutable layer
state/percolation flux ordering under hourly lane execution.
