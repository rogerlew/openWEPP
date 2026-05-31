# HPHYS0216 Residual Gap Matrix

Status: completed
Evidence mode: Static + Ran

## Comparator sources
- Baseline integrated reference:
  `docs/work-packages/20260530-hphys0214-integrated-hold-lift-readjudication-001/artifacts/hphys0214-residual-gap-matrix.md`
- HPHYS0216 rerun summary:
  `/tmp/hphys0216_20260531T053959Z/parity/reports/hillslope_semantic_summary.tsv`

## Family matrix
| Family/Column | HPHYS0214 fails | HPHYS0216 fails | Delta (0216-0214) | HPHYS0216 mean abs diff avg | Status |
| --- | --- | --- | --- | --- | --- |
| `ProfileFCStore` | `27/39` | `39/39` | `+12` | `7.22117381046073` | regressed |
| `Dp` | `39/39` | `39/39` | `0` | `0.26436808916537585` | unchanged-open |
| `latqcc` | `39/39` | `39/39` | `0` | `0.8131880775568225` | unchanged-open |
| `Total-Soil` | `39/39` | `39/39` | `0` | `140.87503038397853` | unchanged-open |
| `SoilWaterTotal` | `39/39` | `39/39` | `0` | `140.87503038397853` | unchanged-open |

## Gap register
| Gap ID | Description | Evidence | Status |
| --- | --- | --- | --- |
| `HP216-GAP-001` | FC layer-authority migration regressed semantic fail count (`27/39 -> 39/39`). | Ran: HPHYS0216 summary TSV | open |
| `HP216-GAP-002` | Cross-family coupled threshold behavior remains unresolved (`Dp`, `latqcc`, `Total-Soil`, `SoilWaterTotal`). | Ran: HPHYS0216 summary TSV | open |
