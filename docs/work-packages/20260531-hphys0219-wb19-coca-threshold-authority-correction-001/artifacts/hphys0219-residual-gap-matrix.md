# HPHYS0219 Residual Gap Matrix

Status: completed
Evidence mode: Static + Ran

## Evidence sources
- HPHYS0219 summary:
  `/tmp/hphys0219_20260531T083756Z/parity/reports/hillslope_semantic_summary.json`
- HPHYS0218 summary (reference):
  `/tmp/hphys0218_20260531T075251Z/parity/reports/hillslope_semantic_summary.json`
- HPHYS0217 summary (reference):
  `/tmp/hphys0217_20260531T071120Z/parity/reports/hillslope_semantic_summary.json`

## Comparator integrity check (Ran)
- `zero_common_row_reports = 0`
- `nonzero_common_row_reports = 39`

| Gap ID | Description | Evidence | Status |
| --- | --- | --- | --- |
| `HP219-GAP-001` | WB19 authority must be corrected to baseline `coca` lineage. | Contracts + production now encode `drfc = fc + (1-coca)*dg`; guards enforce required `coca_####`. | closed |
| `HP219-GAP-002` | `Dp` residual should recover from HPHYS0218 regression. | Fail count unchanged (`39/39`), mean improved `0.3269109066808126 -> 0.2808444379937233`. | improved, open |
| `HP219-GAP-003` | `latqcc` residual should preserve HPHYS0218 improvement. | Fail count unchanged (`39/39`), mean regressed `0.7496847101588174 -> 0.7948120660697657` (still below HPHYS0217 `0.8131880775568228`). | regressed vs 0218, open |
| `HP219-GAP-004` | `Total-Soil` / `SoilWaterTotal` coupled families should not regress. | Fail counts unchanged (`39/39`), means regressed `140.69907071572365 -> 140.82816405718864` (still below HPHYS0217 `140.87503038397858`). | regressed vs 0218, open |
| `HP219-GAP-005` | `ProfileFCStore` control should remain stable. | Unchanged (`27/39`, mean `~2.05269116`). | stable control |

## Summary
- HPHYS0219 closed coefficient-family authority mismatch (`cpm -> coca`) and
  recovered part of the `Dp` regression from HPHYS0218.
- Fail saturation remains unchanged across monitored families.
- Coupled tradeoff remains unresolved (`Dp` improved while `latqcc` and
  total-soil families regressed vs HPHYS0218), so integrated hold-lift remains
  blocked.
