# hillstab06-rerun-delta-report

Status: complete  
Evidence mode: Ran

## Cohort Summary Delta vs HILLSTAB05
- Total evaluated: unchanged (`1185` = `1166 + 19`)
- Pass count:
  - HILLSTAB05: `90`
  - HILLSTAB06: `1185`
  - Delta: `+1095`
- Hold-lift status: `HOLD -> GO`

## Suite Pass Delta
- `wb05b_1166`:
  - HILLSTAB05: `84/1166`
  - HILLSTAB06: `1166/1166`
  - Delta: `+1082`
- `release_gate_watchlist`:
  - HILLSTAB05: `6/19`
  - HILLSTAB06: `19/19`
  - Delta: `+13`

## Residual Family Delta (Targeted)
- `HKERNEL-WB16-PEAK-E-003`: `1094 -> 0` (`-1094`)
- `HS-SIMPIPE-E-001` (`p24`, `tmax<tmin`): `1 -> 0` (`-1`)
- Total failed cases: `1095 -> 0` (`-1095`)

## Interpretation
- HILLSTAB06 fully closed the immediate-next-action residual families from
  HILLSTAB05.
- Broad stability rerun reached complete pass across both cohorts.
