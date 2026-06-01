# HPHYS0234 Residual Authority Gap Matrix

Status: completed  
Evidence mode: Ran

Reference summaries:
- HPHYS0233 summary:
  `/tmp/hphys0233_20260601T211306Z/parity/reports/hillslope_semantic_summary.json`
- HPHYS0234 summary:
  `/tmp/hphys0234_20260601T215019Z/parity/reports/hillslope_semantic_summary.json`

| Column | HPHYS0233 fail_count | HPHYS0234 fail_count | delta_fail_count | HPHYS0233 mean_abs_diff_mean | HPHYS0234 mean_abs_diff_mean | delta_mean_abs_diff_mean |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `Dp` | 39 | 39 | +0 | 0.223504213147 | 0.223504213147 | +0.000000000000 |
| `latqcc` | 39 | 39 | +0 | 0.790397340612 | 0.790397340612 | +0.000000000000 |
| `Total-Soil` | 39 | 39 | +0 | 134.129091721962 | 134.129091721962 | +0.000000000000 |
| `SoilWaterTotal` | 39 | 39 | +0 | 134.129091721962 | 134.129091721962 | +0.000000000000 |
| `ProfileFCStore` | 27 | 27 | +0 | 2.052691160104 | 2.052691160104 | +0.000000000000 |

## Interpretation

1. HPHYS0234 WB13 anti-shadow hardening produced no cohort-level semantic delta
   versus HPHYS0233 for monitored HOLD families.
2. Residual closure remains open (`fail_count > 0`) for all monitored columns.
3. Stream disposition remains `HOLD`.
