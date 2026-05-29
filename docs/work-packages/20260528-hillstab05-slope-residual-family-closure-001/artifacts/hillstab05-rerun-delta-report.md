# hillstab05-rerun-delta-report

Status: complete  
Evidence mode: Ran

## Cohort Summary Delta vs HILLSTAB02
- Total evaluated: unchanged (`1185` = `1166 + 19`)
- Pass count:
  - HILLSTAB02: `0`
  - HILLSTAB05: `90`
- Hold-lift status: unchanged (`HOLD`)

## Suite Pass Delta
- `wb05b_1166`:
  - HILLSTAB02: `0/1166`
  - HILLSTAB05: `84/1166`
- `release_gate_watchlist`:
  - HILLSTAB02: `0/19`
  - HILLSTAB05: `6/19`

## Residual Family Delta (Top)
- slope token parse (`line 7, column 3`): `33 -> 0` (`-33`)
- slope endpoint constraint: `24 -> 0` (`-24`)
- slope cross-OFE boundary mismatch: `11 -> 0` (`-11`)
- `HS-RUNTIME-E-023`: `46 -> 0` (`-46`)
- `HKERNEL-WB16-PEAK-E-003`: `563 -> 1094` (`+531`)
- `HS-SIMPIPE-E-001` (`wb11_seed` tmax<tmin watchlist case): `0 -> 1` (`+1`)

## Interpretation
- HILLSTAB05 fully closed all target slope parser/runtime residual families.
- As expected, broader flow-through now reaches downstream runtime closure
  checks, making WB16 the dominant remaining blocker.
