# hillstab02-remediation-report

Status: complete  
Evidence mode: Ran

## Failure Decomposition (HILLSTAB01 Baseline)
- `SOL-E-006` was dominated by disturbed-policy rows where `luse`/`stext`
  were quoted with embedded spaces.
- `MAN-E-009` was dominated by `tilseq=0` with `nseq>0` legacy sentinel usage.

## Remediation Implemented
1. Soil parser compatibility path now tokenizes quoted policy-row fields for
   disturbed datvers (`9002/9003/9005`) and preserves strict-mode rejection.
2. Management parser compatibility path now accepts `tilseq=0` sentinel when
   `nseq>0`; strict mode remains unchanged.
3. Contract-derived fixtures/tests added for both failure families.

## Rerun Outcome Checks
- `SOL-E-006` log occurrences:
  - HILLSTAB01: `843`
  - HILLSTAB02: `0`
- `MAN-E-009` log occurrences:
  - HILLSTAB01: `93`
  - HILLSTAB02: `0`

## Residual Blocking Families
- `HKERNEL-WB16-PEAK-E-003`: `563` cases (HILLSTAB02)
- `HKERNEL-EROD14-WAVE2-E-003`: `508` cases (HILLSTAB02)
- Slope parse/runtime families remain:
  - token parse (`line 7, col 3`): `33`
  - endpoint constraint: `24`
  - cross-OFE boundary mismatch: `11`
  - `HS-RUNTIME-E-023`: `46`

## Conclusion
- Parser-targeted closure objective achieved (both dominant parser families
  eliminated).
- Stability hold-lift is still blocked by runtime/kernel and slope-family
  failures.
