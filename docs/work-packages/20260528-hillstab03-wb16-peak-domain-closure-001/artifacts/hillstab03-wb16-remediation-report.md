# hillstab03-wb16-remediation-report

Status: complete  
Evidence mode: Ran

## Failure Decomposition (HILLSTAB02 Baseline)
- Dominant WB16 residual entering package:
  - `HKERNEL-WB16-PEAK-E-003`: `563` cases.
- Secondary dominant family:
  - `HKERNEL-EROD14-WAVE2-E-003`: `508` cases.

## Remediation Implemented
1. Canonical WB16 authority amended to remove non-authoritative `timep`
   coupling and align branch partitioning with baseline `appmth.for`.
2. Contract-derived WB16 tests expanded to all authoritative branch selectors,
   including explicit `vstar >= 1` constant-excess branch.
3. Production WB16 runtime updated for:
   - `tc(vstar)` branch separator for `vstar < 1`,
   - explicit branch `4` for `vstar >= 1`,
   - corrected `m` and `vstar` domain posture.

## Rerun Outcome Checks
- `HKERNEL-WB16-PEAK-E-003`:
  - HILLSTAB02: `563`
  - HILLSTAB03: `437`
  - Delta: `-126`
- Cohort pass count moved from `0/1185` (HILLSTAB02) to `24/1185`
  (HILLSTAB03).

## Residual Blocking Families After HILLSTAB03
- `HKERNEL-EROD14-WAVE2-E-003`: `610`
- `HKERNEL-WB16-PEAK-E-003`: `437`
- `HS-RUNTIME-E-023`: `46`
- Slope parse subfamilies:
  - line 7/column 3 token parse: `33`
  - endpoint constraint: `24`
  - cross-OFE boundary mismatch: `11`

## Conclusion
- HILLSTAB03 achieved material WB16 reduction but did not fully eliminate the
  WB16 failure family.
- Hold-lift remains blocked by WB16 residual plus EROD14 and slope/runtime
  families.
