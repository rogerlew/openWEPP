# HPHYS0218 Residual Gap Matrix

Status: completed
Evidence mode: Static + Ran

## Evidence sources
- HPHYS0218 summary:
  `/tmp/hphys0218_20260531T075251Z/parity/reports/hillslope_semantic_summary.json`
- HPHYS0217 summary (reference):
  `/tmp/hphys0217_20260531T071120Z/parity/reports/hillslope_semantic_summary.json`

## Comparator integrity check (Ran)
- `zero_common_row_reports = 0`
- `nonzero_common_row_reports = 39`

| Gap ID | Description | Evidence | Status |
| --- | --- | --- | --- |
| `HP218-GAP-001` | `latqcc` family should improve under `drfc`-adjusted WB19 thresholds. | Fail count unchanged (`39/39`), mean improved `0.8131880775568228 -> 0.7496847101588174`. | improved, open |
| `HP218-GAP-002` | `Dp` family should not regress under WB19 threshold migration. | Fail count unchanged (`39/39`), mean worsened `0.2643680891653757 -> 0.3269109066808126`. | regressed, open |
| `HP218-GAP-003` | Coupled storage families (`Total-Soil`, `SoilWaterTotal`) remain unresolved. | Fail counts unchanged (`39/39` each), mean improved slightly (`140.87503038397858 -> 140.69907071572365`). | open |
| `HP218-GAP-004` | `ProfileFCStore` control should remain non-regressing. | Unchanged (`27/39`, mean `~2.05269116`). | stable control |

## Summary
- HPHYS0218 closed WB19 threshold contract/code lineage and improved `latqcc`
  mean residuals, but did not reduce fail saturation.
- `Dp` regressed in mean absolute diff across all hillslopes and remains the
  primary blocker for coupled hold-lift progression.
