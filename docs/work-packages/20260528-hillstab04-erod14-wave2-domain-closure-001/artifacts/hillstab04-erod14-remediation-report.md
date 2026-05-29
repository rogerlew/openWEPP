# hillstab04-erod14-remediation-report

Status: complete  
Evidence mode: Ran

## Failure Decomposition (Entry Baseline)
- Dominant EROD14 residual entering package (from HILLSTAB03):
  - `HKERNEL-EROD14-WAVE2-E-003`: `610` cases.
- Dominant branch trigger observed in decomposed rerun logs:
  - clipping reproportion branch with `ratbot = 0` (all-class `sedmax`
    saturation path) hard-failing in openWEPP runtime.

## Remediation Implemented
1. Canonical `SC-SED-001` authority amended to require baseline
   `enrich.for` all-class saturation semantics for `ratbot = 0`.
2. Contract-derived EROD14 integration vector updated from expected failure to
   expected successful closure under all-class `sedmax` saturation.
3. Production wave-2 kernel branch updated to re-enter clipping loop for
   `ratbot = 0` instead of emitting non-authoritative domain failure.

## Rerun Outcome Checks
- `HKERNEL-EROD14-WAVE2-E-003`:
  - HILLSTAB02: `508`
  - HILLSTAB03: `610`
  - HILLSTAB04: `0`
  - Delta vs HILLSTAB02: `-508`
  - Delta vs HILLSTAB03: `-610`
- Cohort pass count moved from:
  - `0/1185` (HILLSTAB02) to `76/1185` (HILLSTAB04),
  - `24/1185` (HILLSTAB03) to `76/1185` (HILLSTAB04).

## Residual Blocking Families After HILLSTAB04
- `HKERNEL-WB16-PEAK-E-003`: `994`
- `HS-RUNTIME-E-023`: `46`
- slope parse subfamilies:
  - line 7/column 3 token parse: `33`
  - endpoint constraint: `24`
  - cross-OFE boundary mismatch: `11`

## Conclusion
- HILLSTAB04 achieved complete closure of the targeted EROD14 residual family.
- Hold-lift remains blocked by non-target residual families, now dominated by
  WB16 runtime failures.
