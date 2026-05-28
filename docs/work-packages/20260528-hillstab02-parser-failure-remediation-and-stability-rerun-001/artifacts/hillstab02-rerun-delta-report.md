# hillstab02-rerun-delta-report

Status: complete  
Evidence mode: Ran

## Cohort Summary Delta vs HILLSTAB01
- Total evaluated: unchanged (`1185` = `1166 + 19`)
- Pass count:
  - HILLSTAB01: `0`
  - HILLSTAB02: `0`
- Hold-lift status: unchanged (`HOLD`)

## CLI Family Shift
- `CLIHILL-E-010`:
  - HILLSTAB01: `1004`
  - HILLSTAB02: `68`
  - Delta: `-936`
- `CLIHILL-E-011`:
  - HILLSTAB01: `181`
  - HILLSTAB02: `1117`
  - Delta: `+936`

## Targeted Parser Family Delta
- `SOL-E-006`: `843 -> 0` (resolved)
- `MAN-E-009`: `93 -> 0` (resolved)

## Residual Family Delta (Top)
- `HKERNEL-WB16-PEAK-E-003`: `140 -> 563` (+423)
- `HKERNEL-EROD14-WAVE2-E-003`: `40 -> 508` (+468)
- `HS-RUNTIME-E-023`: `1 -> 46` (+45)
- slope token parse (`line 7 col 3`): `33 -> 33` (no change)
- slope endpoint constraint: `24 -> 24` (no change)
- slope cross-OFE boundary mismatch: `11 -> 11` (no change)

## Interpretation
- HILLSTAB02 successfully removed parser compatibility blockers from the top
  failure stack.
- With parser closure in place, runtime/kernel and slope invariants now dominate
  observed failures across the same cohorts.
