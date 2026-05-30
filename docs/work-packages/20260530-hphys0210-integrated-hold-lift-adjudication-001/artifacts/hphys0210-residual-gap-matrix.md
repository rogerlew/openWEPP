# HPHYS0210 Residual Gap Matrix

Status: completed  
Evidence mode: Static + Ran

## Integrated family matrix
Source: `/tmp/hphys0210_20260530T194829Z/diagnostics/hphys0210_integrated_family_summary.tsv`

| Family/Column | Fail Hillslopes | Mean Abs Diff Avg | Delta vs HPHYS0207 | Contract Status | Confidence Tier | Integrated Status |
| --- | --- | --- | --- | --- | --- | --- |
| `ProfileDepth` | `0/39` | `0.0000` | `+0.0000` | closed-process-authoritative | single-OFE daily WAT (higher-confidence) | closed |
| `ProfilePorosityCap` | `0/39` | `0.0209` | `+0.0000` | closed-process-authoritative | single-OFE daily WAT (higher-confidence) | closed |
| `ProfileWPStore` | `1/39` | `0.0573` | `+0.0000` | bounded-expected-process-correct-diagnostic | single-OFE daily WAT (higher-confidence) | bounded-open (`H7`) |
| `ProfileFCStore` | `27/39` | `2.0527` | `+0.0000` | open-coupled-threshold-lineage | single-OFE daily WAT (higher-confidence) | open |
| `Dp` | `39/39` | `40.1559` | `+39.9689` | open-coupled-threshold-lineage | single-OFE daily WAT (higher-confidence) | open (regressed magnitude) |
| `latqcc` | `39/39` | `173.2285` | `+89.6728` | open-coupled-threshold-lineage | single-OFE daily WAT (higher-confidence) | open (regressed magnitude) |
| `Total-Soil` | `39/39` | `116.0649` | `-6.1036` | open-coupled-threshold-lineage | single-OFE daily WAT (higher-confidence) | open |
| `SoilWaterTotal` | `39/39` | `116.0649` | `-6.1036` | open-coupled-threshold-lineage | single-OFE daily WAT (higher-confidence) | open |

## Gap register
| Gap ID | Description | Evidence | Status |
| --- | --- | --- | --- |
| `HP210-GAP-001` | Coupled threshold-lineage residual families (`ProfileFCStore`, `Dp`, `latqcc`, `Total-Soil`, `SoilWaterTotal`) remain open after HPHYS0208 closure wave. | Ran: integrated family summary and HPHYS0208 disposition. | open |
| `HP210-GAP-002` | `ProfileWPStore` remains near-closed (`1/39`) and bounded to `H7`; requires integrated tracking but not local rollback of HPHYS0209 adjudication. | Ran + Static: HPHYS0209 focus summary and disposition. | bounded-open |
| `HP210-GAP-003` | Integrated hold-lift cannot be justified while higher-confidence daily WAT lane still shows saturated fail-count blockers in coupled families. | Ran: integrated matrix + HPHYS0208/0209 evidence intake. | open |
