# HPHYS0214 Residual Gap Matrix

Status: completed  
Evidence mode: Static + Ran

## Integrated family matrix
Source:
`/tmp/hphys0214_20260531T004200Z/diagnostics/hphys0214_integrated_family_summary.tsv`

| Family/Column | Fail Hillslopes (HP212 -> HP213) | Mean Abs Diff Avg (HP212 -> HP213) | Delta | Contract Status | Confidence Tier | Integrated Status |
| --- | --- | --- | --- | --- | --- | --- |
| `ProfileFCStore` | `26/38 -> 27/39` | `2.03798 -> 2.05269` | `+0.01471` | open-coupled-threshold-lineage | single-OFE daily WAT (higher-confidence) | open |
| `Dp` | `38/38 -> 39/39` | `0.26440 -> 0.26437` | `-0.00003` | open-coupled-threshold-lineage | single-OFE daily WAT (higher-confidence) | open |
| `latqcc` | `38/38 -> 39/39` | `0.95829 -> 0.81319` | `-0.14511` | open-coupled-threshold-lineage | single-OFE daily WAT (higher-confidence) | open |
| `Total-Soil` | `38/38 -> 39/39` | `141.25575 -> 140.87503` | `-0.38072` | open-coupled-threshold-lineage | single-OFE daily WAT (higher-confidence) | open |
| `SoilWaterTotal` | `38/38 -> 39/39` | `141.25575 -> 140.87503` | `-0.38072` | open-coupled-threshold-lineage | single-OFE daily WAT (higher-confidence) | open |

## H5 blocker continuity check
- HPHYS0212: `HKERNEL-WB12-STORAGE-E-003` present.
- HPHYS0213: `HKERNEL-WB12-STORAGE-E-003` absent.
- Interpretation: runtime execution blocker is closed, but integrated monitored
  residual families remain open.

## Gap register
| Gap ID | Description | Evidence | Status |
| --- | --- | --- | --- |
| `HP214-GAP-001` | Monitored residual families remain open at integrated readjudication (`ProfileFCStore`, `Dp`, `latqcc`, `Total-Soil`, `SoilWaterTotal`). | Ran: `/tmp/hphys0214_20260531T004200Z/diagnostics/hphys0214_integrated_family_summary.json` | open |
| `HP214-GAP-002` | No process-authority evidence yet supports hold-lift for monitored families despite H5 blocker closure. | Static + Ran: HPHYS0211/0212/0213 dispositions + HPHYS0214 diagnostics. | open |
