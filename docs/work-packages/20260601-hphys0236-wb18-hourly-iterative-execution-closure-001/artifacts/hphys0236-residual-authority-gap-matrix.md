# HPHYS0236 Residual Authority Gap Matrix

Status: completed  
Evidence mode: Ran

## Evidence Sources

- Current rerun summary:
  `/tmp/hphys0236_20260601T230600Z/parity/reports/hillslope_semantic_summary.json`
- Prior comparison anchor (HPHYS0234 summary):
  `/tmp/hphys0234_20260601T215019Z/parity/reports/hillslope_semantic_summary.json`

## Monitored-Family Readjudication (`H1..H39`)

| Family | HPHYS0234 mean_abs_diff_mean | HPHYS0236 mean_abs_diff_mean | Delta | Fail count |
| --- | ---: | ---: | ---: | ---: |
| `Dp` | `0.22350421314678484` | `0.28852654343234196` | `+0.06502233028555712` | `39/39` |
| `latqcc` | `0.7903973406116435` | `0.7856380578243605` | `-0.004759282787282992` | `39/39` |
| `Total-Soil` | `134.12909172196171` | `140.70750455380795` | `+6.5784128318462365` | `39/39` |
| `SoilWaterTotal` | `134.12909172196171` | `140.70750455380795` | `+6.5784128318462365` | `39/39` |
| `ProfileFCStore` | `2.0526911601041165` | `2.052691160104116` | `~0.0` | `27/39` |

## Gap Decision

1. HPHYS0236 successfully migrated WB18 hourly iterative execution shape.
2. This slice did not close monitored residual families; `Dp`,
   `Total-Soil`, and `SoilWaterTotal` worsened versus HPHYS0234.
3. Hold-lift remains blocked pending next authority slice for coupled hourly
   forcing/infiltration/percolation behavior.
