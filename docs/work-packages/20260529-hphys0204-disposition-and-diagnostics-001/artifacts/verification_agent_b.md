# HPHYS0204 Verification Agent B

Status: completed  
Evidence mode: Ran

## Verification checks
1. Recomputed targeted fail-hillslope counts from:
   `/tmp/hphys0207_20260530T042607Z/parity/reports/semantic/H*.semantic.json`.
2. Recomputed targeted mean-abs-diff averages for each tracked column family.
3. Revalidated summary-level common-row continuity and cohort cardinality from:
   `/tmp/hphys0207_20260530T042607Z/parity/reports/hillslope_semantic_summary.json`.

## Confirmed outcomes
- Summary cardinality:
  - `total_hillslopes=39`
  - `total_common_rows=56979`
  - `semantic_fail_count=39`
- Targeted fail-hillslope counts:
  - `Dp 39`, `latqcc 39`, `Total-Soil 39`, `SoilWaterTotal 39`
  - `ProfileDepth 0`, `ProfilePorosityCap 0`
  - `ProfileFCStore 27`, `ProfileWPStore 1`
- Targeted mean-abs-diff averages:
  - `Dp 0.187018`, `latqcc 83.555731`
  - `Total-Soil 122.168462`, `SoilWaterTotal 122.168462`
  - `ProfileDepth 0.000000`, `ProfilePorosityCap 0.020913`
  - `ProfileFCStore 2.052691`, `ProfileWPStore 0.057297`

## Verdict
- Integrated residual diagnostics in HPHYS0204 are reproducible.
- Disposition `HOLD` is evidence-consistent.
