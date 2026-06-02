# HPHYS0244 Disposition

Static: disposition from package artifact evidence.
Ran: H1/H7/H39 targeted WAT diagnostics, emitted-surface audit, and source
lineage review.

## Decision
`HOLD_PENDING_WB11_WB18_LAYER_TELEMETRY_AND_MUTABLE_STORAGE_CLOSURE`

## Closure Measures
1. `MEASURE-HP244-001`: satisfied. `H1`, `H7`, and `H39` baseline/candidate
   WAT comparisons were generated for `Dp`, `Total-Soil`, and
   `SoilWaterTotal`.
2. `MEASURE-HP244-002`: satisfied with blocker. Current emitted artifacts do
   not expose layer `st`/`theta` or WB18 `Pe`; this is recorded as a
   diagnostics observability blocker, not a completed layer-state comparison.
3. `MEASURE-HP244-003`: satisfied. Prior HPHYS root-cause evidence was reviewed
   and matches the current slice.
4. `MEASURE-HP244-004`: satisfied. Recommendations distinguish direct WAT
   output evidence from static lineage inference.

## Final Assessment
HPHYS0244 confirms the post-HPHYS0243 focus should remain hillslope
water-balance storage, specifically WB11/WB18 mutable storage continuity and
percolation flux writeback/observability.

The most important residual signal is not full-period `Dp` magnitude. It is the
first-week overdrain plus simultaneous deep storage deficit:
- `H1`: day-1 `Dp` delta `+44.004399 mm`, day-1..7 `Total-Soil` mean delta
  `-191.819552 mm`.
- `H7`: day-1 `Dp` delta `+33.612610 mm`, day-1..7 `Total-Soil` mean delta
  `-171.992073 mm`.
- `H39`: day-1 `Dp` delta `+22.740342 mm`, day-1..7 `Total-Soil` mean delta
  `-210.062828 mm`.

Existing artifacts do not support a direct statement about per-layer `st` or
`theta` trajectories. The correct next move is a telemetry-first package that
captures `wb18_perc_theta_*`, `wb18_perc_pei_*`, `D`, `Pe`, and
`wb11_soil_water` at scheduler phase boundaries for `H1`, `H7`, and `H39`.

## Production Code
No production code was modified.

## Follow-On
Recommended next package:
`20260602-hphys0245-wb11-wb18-layer-telemetry-and-storage-continuity-001`.
