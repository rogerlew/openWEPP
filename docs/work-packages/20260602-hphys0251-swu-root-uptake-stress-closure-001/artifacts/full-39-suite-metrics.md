# Full 39 Hillslope Suite Metrics

Status: complete

Evidence mode: ran

Ran:

- Runtime root: `/tmp/hphys0251_20260602T184933Z`.
- Runtime status file:
  `/tmp/hphys0251_20260602T184933Z/reports/hillslope_batch_status.tsv`.
- Semantic status file:
  `/tmp/hphys0251_20260602T184933Z/reports/semantic_status.tsv`.
- Semantic summary JSON:
  `/tmp/hphys0251_20260602T184933Z/reports/hillslope_semantic_summary.json`.
- Semantic summary Markdown:
  `/tmp/hphys0251_20260602T184933Z/reports/hillslope_semantic_summary.md`.
- Delta Markdown:
  `/tmp/hphys0251_20260602T184933Z/reports/hphys0251_delta_from_hphys0250.md`.

Summary:

# HPHYS0251 Full 39 Semantic Summary

- Root: `/tmp/hphys0251_20260602T184933Z`
- Reports: 39
- Semantic pass: 0/39

| Symbol | Pass Hillslopes | Total Fail Count | Mean Abs Diff Mean | Max Abs Diff | Worst |
|---|---:|---:|---:|---:|---|
| Dp | 0/39 | 40811 | 0.172318 | 0.24 | H1 [1, 1, 2014] |
| Ep | 0/39 | 56505 | 1.70276 | 7.77999 | H13 [1, 202, 2015] |
| Er | 39/39 | 0 | 0 | 0 | None None |
| Es | 38/39 | 1176 | 0.0187496 | 1.89 | H6 [1, 204, 2014] |
| P | 39/39 | 0 | 4.56799e-17 | 3.55271e-15 | H1 [1, 64, 2014] |
| ProfileFCStore | 12/39 | 39447 | 2.05269 | 9.33443 | H7 [1, 1, 2013] |
| ProfileWPStore | 38/39 | 1461 | 0.0572975 | 1.66986 | H7 [1, 1, 2013] |
| Q | 0/39 | 2986 | 0.925027 | 194.716 | H6 [1, 65, 2014] |
| QOFE | 0/39 | 2980 | 0.925027 | 194.716 | H6 [1, 65, 2014] |
| RM | 0/39 | 10678 | 2.3018 | 204.851 | H6 [1, 65, 2014] |
| Snow-Water | 0/39 | 24137 | 58.1957 | 562.47 | H9 [1, 114, 2014] |
| SoilWaterTotal | 0/39 | 56979 | 170.349 | 620.616 | H24 [1, 148, 2014] |
| Total-Soil | 0/39 | 56979 | 170.349 | 620.616 | H24 [1, 148, 2014] |
| frozwt | 39/39 | 0 | 0 | 0 | None None |
| latqcc | 0/39 | 39839 | 0.802705 | 36.7496 | H33 [1, 2, 2013] |

Delta from HPHYS0250:

| Symbol | HPHYS0250 Fail Count | HPHYS0251 Fail Count | Fail Δ | HPHYS0250 Mean Abs Diff Mean | HPHYS0251 Mean Abs Diff Mean | Mean Δ |
|---|---:|---:|---:|---:|---:|---:|
| Ep | 56230 | 56505 | 275 | 1.68341 | 1.70276 | 0.0193505 |
| Total-Soil | 56955 | 56979 | 24 | 168.131 | 170.349 | 2.21855 |
| SoilWaterTotal | 56955 | 56979 | 24 | 168.131 | 170.349 | 2.21855 |
| Dp | 40512 | 40811 | 299 | 0.171118 | 0.172318 | 0.00120031 |
| Es | 1165 | 1176 | 11 | 0.0186939 | 0.0187496 | 5.5709e-05 |
| Q | 2986 | 2986 | 0 | 0.925027 | 0.925027 | ~0 |
| RM | 10678 | 10678 | 0 | 2.3018 | 2.3018 | ~0 |
| Snow-Water | 24137 | 24137 | 0 | 58.1957 | 58.1957 | ~0 |

Conclusion:

- Full-suite semantic status remains `HOLD`: `0/39` hillslopes pass.
- HPHYS0251 made the intended SWU lineage observable and contract-tested, but
  it did not materially reduce full-suite `Ep` or aggregate-storage residuals.
