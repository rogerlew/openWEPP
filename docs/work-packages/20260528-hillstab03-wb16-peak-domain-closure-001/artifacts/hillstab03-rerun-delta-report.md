# hillstab03-rerun-delta-report

Status: complete  
Evidence mode: Ran

## Cohort Summary Delta vs HILLSTAB02
- Total evaluated: unchanged (`1185` = `1166 + 19`)
- Pass count:
  - HILLSTAB02: `0`
  - HILLSTAB03: `24`
- Hold-lift status: unchanged (`HOLD`).

## Suite Pass Delta
- `wb05b_1166`:
  - HILLSTAB02: `0/1166`
  - HILLSTAB03: `20/1166`
- `release_gate_watchlist`:
  - HILLSTAB02: `0/19`
  - HILLSTAB03: `4/19`

## Residual Family Delta (Top)
- `HKERNEL-WB16-PEAK-E-003`: `563 -> 437` (`-126`)
- `HKERNEL-EROD14-WAVE2-E-003`: `508 -> 610` (`+102`)
- `HS-RUNTIME-E-023`: `46 -> 46` (no change)
- slope token parse (`line 7, column 3`): `33 -> 33` (no change)
- slope endpoint constraint: `24 -> 24` (no change)
- slope cross-OFE boundary mismatch: `11 -> 11` (no change)

## Interpretation
- WB16-targeted remediation produced measurable reduction in WB16 runtime
  failures and allowed some cases to complete successfully.
- As expected from flow-through exposure, EROD14 failures increased as more
  cases advanced beyond WB16 gating.
- Hold-lift remains blocked pending follow-on closure packages for EROD14 and
  slope/runtime residual families.
