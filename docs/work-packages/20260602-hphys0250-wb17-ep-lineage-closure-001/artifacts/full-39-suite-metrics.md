# Full 39 Hillslope Suite Metrics

Status: complete

Evidence mode: ran

Ran:

- Runtime root: `/tmp/hphys0250_20260602T175731Z`.
- Runtime status file: `/tmp/hphys0250_20260602T175731Z/reports/hillslope_batch_status.tsv`.
- Semantic status file: `/tmp/hphys0250_20260602T175731Z/reports/semantic_status.tsv`.
- Semantic summary JSON: `/tmp/hphys0250_20260602T175731Z/reports/hillslope_semantic_summary.json`.
- Semantic summary Markdown: `/tmp/hphys0250_20260602T175731Z/reports/hillslope_semantic_summary.md`.

Summary:

# HPHYS0250 Full 39 Semantic Summary

- Root: `/tmp/hphys0250_20260602T175731Z`
- Reports: 39
- Semantic pass: 0/39

| Symbol | Pass Hillslopes | Total Fail Count | Mean Abs Diff Mean | Max Abs Diff | Worst |
|---|---:|---:|---:|---:|---|
| Dp | 0/39 | 40512 | 0.171118 | 0.24 | H1 [1, 1, 2014] |
| Ep | 0/39 | 56230 | 1.68341 | 7.77843 | H13 [1, 202, 2015] |
| Er | 39/39 | 0 | 0 | 0 | None None |
| Es | 38/39 | 1165 | 0.0186939 | 1.89 | H6 [1, 204, 2014] |
| P | 39/39 | 0 | 4.56799e-17 | 3.55271e-15 | H1 [1, 64, 2014] |
| ProfileFCStore | 12/39 | 39447 | 2.05269 | 9.33443 | H7 [1, 1, 2013] |
| ProfileWPStore | 38/39 | 1461 | 0.0572975 | 1.66986 | H7 [1, 1, 2013] |
| Q | 0/39 | 2986 | 0.925027 | 194.716 | H6 [1, 65, 2014] |
| QOFE | 0/39 | 2980 | 0.925027 | 194.716 | H6 [1, 65, 2014] |
| RM | 0/39 | 10678 | 2.3018 | 204.851 | H6 [1, 65, 2014] |
| Snow-Water | 0/39 | 24137 | 58.1957 | 562.47 | H9 [1, 114, 2014] |
| SoilWaterTotal | 0/39 | 56955 | 168.131 | 619.185 | H24 [1, 148, 2014] |
| Total-Soil | 0/39 | 56955 | 168.131 | 619.185 | H24 [1, 148, 2014] |
| frozwt | 39/39 | 0 | 0 | 0 | None None |
| latqcc | 0/39 | 39830 | 0.80271 | 36.7523 | H33 [1, 2, 2013] |

Delta from HPHYS0249:

| Symbol | HPHYS0249 | HPHYS0250 | Interpretation |
|---|---:|---:|---|
| `Ep` fail count | 56834 | 56230 | Improved by 604 rows; all 39 hillslopes still fail. |
| `Ep` mean abs diff mean | 1.739422 | 1.683414 | Small reduction; residual remains dominant. |
| `Ep` max abs diff | 7.780000 | 7.778432 | Essentially unchanged worst magnitude. |
| `Total-Soil` mean abs diff mean | 131.293228 | 168.130627 | Storage worsened after real Ep withdrawal became active. |
| `SoilWaterTotal` mean abs diff mean | 131.293228 | 168.130627 | Same as Total-Soil alias family. |

Conclusion:

- HPHYS0250 corrected the missing/zero `Ep` lineage class: H1 now emits
  nonzero `Ep` on 1461/1461 rows.
- HPHYS0250 did not close semantic parity: remaining `Ep` residual is an
  under-uptake/stress magnitude defect, not a publication-ingestion defect.
