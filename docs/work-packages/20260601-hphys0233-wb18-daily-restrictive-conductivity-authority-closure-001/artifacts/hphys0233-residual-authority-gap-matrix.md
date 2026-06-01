# HPHYS0233 Residual Authority Gap Matrix

Status: completed  
Evidence mode: Ran

Reference summaries:
- HPHYS0232 summary:
  `/tmp/hphys0232_20260601T201921Z/parity/reports/hillslope_semantic_summary.json`
- HPHYS0233 summary:
  `/tmp/hphys0233_20260601T211306Z/parity/reports/hillslope_semantic_summary.json`

| Column | HPHYS0232 fail_count | HPHYS0233 fail_count | delta_fail_count | HPHYS0232 mean_abs_diff_mean | HPHYS0233 mean_abs_diff_mean | delta_mean_abs_diff_mean |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `Dp` | 39 | 39 | +0 | 0.324013370329 | 0.223504213147 | -0.100509157182 |
| `latqcc` | 39 | 39 | +0 | 0.751767554749 | 0.790397340612 | +0.038629785862 |
| `Total-Soil` | 39 | 39 | +0 | 140.650907919539 | 134.129091721962 | -6.521816197577 |
| `SoilWaterTotal` | 39 | 39 | +0 | 140.650907919539 | 134.129091721962 | -6.521816197577 |
| `ProfileFCStore` | 27 | 27 | +0 | 2.052691160104 | 2.052691160104 | +0.000000000000 |

## Interpretation

1. `Dp` and aggregate storage families improved materially at cohort mean level.
2. `latqcc` regressed, indicating cross-family coupling remains unresolved.
3. `ProfileFCStore` remains unchanged and still fails on `27/39` reports.
4. No monitored family reached zero-fail closure; stream remains `HOLD`.
