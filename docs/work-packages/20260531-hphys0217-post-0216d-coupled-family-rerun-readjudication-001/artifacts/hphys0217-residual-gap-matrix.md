# HPHYS0217 Residual Gap Matrix

Status: completed
Evidence mode: Static + Ran

## Evidence sources
- HPHYS0217 summary:
  `/tmp/hphys0217_20260531T071120Z/parity/reports/hillslope_semantic_summary.json`
- HPHYS0216 summary (reference):
  `/tmp/hphys0216_20260531T053959Z/parity/reports/hillslope_semantic_summary.json`

| Gap ID | Description | Evidence | Status |
| --- | --- | --- | --- |
| `HP217-GAP-001` | `ProfileFCStore` must be re-evaluated after HPHYS0216D fix. | Ran summary shows `39/39 -> 27/39` and `7.22117381046073 -> 2.052691160104116`. | improved, not closed |
| `HP217-GAP-002` | `Dp` remains fail-saturated across cohort. | Ran summary: `39/39`, `0.2643680891653757` (unchanged vs HPHYS0216). | open |
| `HP217-GAP-003` | `latqcc` remains fail-saturated across cohort. | Ran summary: `39/39`, `0.8131880775568228` (unchanged vs HPHYS0216). | open |
| `HP217-GAP-004` | `Total-Soil` remains fail-saturated across cohort. | Ran summary: `39/39`, `140.87503038397858` (unchanged vs HPHYS0216). | open |
| `HP217-GAP-005` | `SoilWaterTotal` remains fail-saturated across cohort. | Ran summary: `39/39`, `140.87503038397858` (unchanged vs HPHYS0216). | open |

## Readjudication outcome
- HPHYS0216D FC authority regression is remediated (control improved).
- Coupled hydrology/storage families remain unresolved and require follow-on
  contract-first remediation.
