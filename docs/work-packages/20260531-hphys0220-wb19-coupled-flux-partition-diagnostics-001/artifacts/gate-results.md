# HPHYS0220 Gate Results

Status: completed
Evidence mode: Ran

## Commands run
1. Coupled delta analysis script over HPHYS0218/0219 semantic reports (pass):
   - computes per-hillslope `Dp`, `latqcc`, `Total-Soil` mean-abs-diff deltas.
2. Correlation analysis script across 39 hillslopes (pass):
   - computes Pearson correlation for cross-family delta vectors.
3. Static lineage audit commands (pass):
   - `rg`/`sed` over baseline `watbal.for` and openWEPP WB19 kernel sources.

## Execution artifacts
- HPHYS0218 summary:
  `/tmp/hphys0218_20260531T075251Z/parity/reports/hillslope_semantic_summary.json`
- HPHYS0219 summary:
  `/tmp/hphys0219_20260531T083756Z/parity/reports/hillslope_semantic_summary.json`
- Baseline source:
  `/workdir/wepp-forest_260430_baseline/src/watbal.for`

## Integrity note
- Package is diagnostics-only and intentionally does not modify production code;
  workspace compile/test gates were not rerun in this package.
