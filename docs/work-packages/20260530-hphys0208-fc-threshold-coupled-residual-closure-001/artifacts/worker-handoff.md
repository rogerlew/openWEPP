# HPHYS0208 Worker Handoff

Status: completed  
Evidence mode: Static + Ran

## Immediate next actions
1. Open follow-on package for coupled residual root-cause decomposition focused
   on unchanged fail counts and Dp/latqcc residual-magnitude regression.
2. Partition follow-on analysis by family:
   - WB18/WB19 threshold consumers affecting `Dp` and `latqcc`,
   - WB13 aggregate publication continuity affecting `Total-Soil` and
     `SoilWaterTotal`,
   - FC publication residual persistence (`ProfileFCStore` `27/39`).
3. Preserve HPHYS0208 contract posture:
   - keep WB11 seed coupling on `sat`/`por_####`/`cpm_####`/`thetfc_####`/
     `thetdr_####`/`dg_####`,
   - keep typed fail-closed guards (no fallback or clamping policy softening),
   - keep WB14 compatibility branch that accepts both HPHYS0208 and legacy
     FC/WP layout semantics.
4. Keep comparator lane continuity fixed:
   - cohort: unpalatable-rind `H1..H39`
   - candidate-year offset: `2012`
   - tolerance file:
     `tools/legacy_comparison_suite/configs/pl14s_wat_tolerances.json`

## Handoff evidence bundle
- Gate logs: `/tmp/hphys0208_20260530T155837Z/gates/`
- Rerun root: `/tmp/hphys0208_20260530T155837Z/parity/`
- Summary:
  `/tmp/hphys0208_20260530T155837Z/parity/reports/hillslope_semantic_summary.json`
- Comparator reports:
  `/tmp/hphys0208_20260530T155837Z/parity/reports/semantic/H*.semantic.json`
- Predecessor comparator reference:
  `/tmp/hphys0207_20260530T042607Z/parity/reports/semantic/H*.semantic.json`
