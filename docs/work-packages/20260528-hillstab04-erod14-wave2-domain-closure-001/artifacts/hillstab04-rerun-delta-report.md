# hillstab04-rerun-delta-report

Status: complete  
Evidence mode: Ran

## Cohort Summary Delta vs HILLSTAB02
- Total evaluated: unchanged (`1185` = `1166 + 19`)
- Pass count:
  - HILLSTAB02: `0`
  - HILLSTAB04: `76`
- Hold-lift status: unchanged (`HOLD`).

## Suite Pass Delta
- `wb05b_1166`:
  - HILLSTAB02: `0/1166`
  - HILLSTAB04: `71/1166`
- `release_gate_watchlist`:
  - HILLSTAB02: `0/19`
  - HILLSTAB04: `5/19`

## Residual Family Delta (Top)
- `HKERNEL-EROD14-WAVE2-E-003`: `508 -> 0` (`-508`)
- `HKERNEL-WB16-PEAK-E-003`: `563 -> 994` (`+431`)
- `HS-RUNTIME-E-023`: `46 -> 46` (no change)
- slope token parse (`line 7, column 3`): `33 -> 33` (no change)
- slope endpoint constraint: `24 -> 24` (no change)
- slope cross-OFE boundary mismatch: `11 -> 11` (no change)

## Interpretation
- HILLSTAB04 fully closed its targeted EROD14 family and materially improved
  broad cohort pass counts.
- As expected for flow-through closure, WB16 now dominates residual runtime
  failures and remains a primary hold-lift blocker.
