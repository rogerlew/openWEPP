# HPHYS0230 Residual Authority Gap Matrix

Status: completed  
Evidence mode: Ran

Reference summaries:
- baseline comparator snapshot:
  `/tmp/hphys0229_20260601T175346Z/parity/reports/hillslope_semantic_summary.json`
- HPHYS0230 summary:
  `/tmp/hphys0230_20260601T183925Z/parity/reports/hillslope_semantic_summary.json`

| Column | HPHYS0229 fail_count | HPHYS0230 fail_count | delta_fail_count | HPHYS0229 mean_abs_diff_mean | HPHYS0230 mean_abs_diff_mean | delta_mean_abs_diff_mean |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `Dp` | 39 | 38 | -1 | 0.32491959785932106 | 0.3244800457269391 | -0.000439552132 |
| `latqcc` | 39 | 38 | -1 | 0.7517675547493208 | 0.75389969923379 | +0.002132144484 |
| `Total-Soil` | 39 | 38 | -1 | 140.7089613428272 | 140.6912996976993 | -0.017661645128 |
| `SoilWaterTotal` | 39 | 38 | -1 | 140.7089613428272 | 140.6912996976993 | -0.017661645128 |
| `ProfileFCStore` | 27 | 26 | -1 | 2.0526911601041165 | 1.8610665559137674 | -0.191624604190 |

## Interpretation

1. Apparent fail-count reductions are structural (`H7` missing candidate WAT),
   not demonstrated residual-family closure.
2. WB18 overdrainage behavior remains materially open (`Dp`/`Total-Soil`
   signatures persist in `H1` acceptance trace).
3. Stream remains in `HOLD` pending follow-on remediation.
