# HPHYS0231 Residual Authority Gap Matrix

Status: completed  
Evidence mode: Ran

Reference summaries:
- HPHYS0230 summary:
  `/tmp/hphys0230_20260601T183925Z/parity/reports/hillslope_semantic_summary.json`
- HPHYS0231 summary:
  `/tmp/hphys0231_20260601T193448Z/parity/reports/hillslope_semantic_summary.json`

| Column | HPHYS0230 fail_count | HPHYS0231 fail_count | delta_fail_count | HPHYS0230 mean_abs_diff_mean | HPHYS0231 mean_abs_diff_mean | delta_mean_abs_diff_mean |
| --- | ---: | ---: | ---: | ---: | ---: | ---: |
| `Dp` | 38 | 39 | +1 | 0.3244800457269391 | 0.3240133703289665 | -0.000466675398 |
| `latqcc` | 38 | 39 | +1 | 0.75389969923379 | 0.7517675547493208 | -0.002132144484 |
| `Total-Soil` | 38 | 39 | +1 | 140.6912996976993 | 140.6509079195388 | -0.040391778160 |
| `SoilWaterTotal` | 38 | 39 | +1 | 140.6912996976993 | 140.6509079195388 | -0.040391778160 |
| `ProfileFCStore` | 26 | 27 | +1 | 1.8610665559137674 | 2.0526911601041165 | +0.191624604190 |

## Interpretation

1. `fail_count` increases by `+1` reflect recovered `H7` coverage
   (`39/39` candidate/comparator reports) rather than regression in execution
   completeness.
2. Residual WB18 overdrainage family remains open; HPHYS0231 closes H7 guard
   recovery and readjudication coverage, not transient `Dp` parity.
3. Stream-level `HOLD` remains appropriate pending WB18 transient remediation.
