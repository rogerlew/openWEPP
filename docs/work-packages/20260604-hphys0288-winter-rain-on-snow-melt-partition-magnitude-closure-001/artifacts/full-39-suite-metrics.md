# Full H1..H39 Suite Metrics

Status: complete
Evidence mode: Ran

# HPHYS0288 Full H1..H39 Semantic Summary

Ran:

- Root: `/tmp/hphys0288_full_release_final_v13_20260604T163204Z`
- Runtime status: `/tmp/hphys0288_full_release_final_v13_20260604T163204Z/reports/hillslope_batch_status.tsv`
- Semantic status: `/tmp/hphys0288_full_release_final_v13_20260604T163204Z/reports/semantic_status.tsv`
- Semantic pass: `0/39`

| Symbol | Pass Hillslopes | Total Fail Count | Mean Abs Diff Mean | Max Abs Diff |
| --- | --- | ---: | ---: | ---: |
| Ep | 0/39 | 45401 | 0.727061 | 7.242659 |
| Total-Soil | 0/39 | 52521 | 57.069194 | 348.886998 |
| SoilWaterTotal | 0/39 | 52521 | 57.069194 | 348.886998 |
| Dp | 1/39 | 9220 | 0.042845 | 0.244800 |
| latqcc | 0/39 | 36003 | 0.373461 | 11.865076 |
| Q | 0/39 | 2108 | 0.552220 | 38.472185 |
| RM | 0/39 | 6633 | 0.248018 | 27.960000 |
| Snow-Water | 0/39 | 10391 | 2.899431 | 65.506840 |

## Delta From HPHYS0287

Ran:

- HPHYS0287 comparison root: `/tmp/hphys0287_full_release_after_review_20260604T221027Z`

| Symbol | Fail Count Delta | Mean Abs Diff Delta | Max Abs Diff Delta | Direction |
| --- | ---: | ---: | ---: | --- |
| Ep | -974 | -0.015602 | -0.085801 | improved |
| Total-Soil | -2363 | -4.046006 | -16.131268 | improved |
| SoilWaterTotal | -2363 | -4.046006 | -16.131268 | improved |
| Dp | +116 | +0.000717 | +0.000000 | worsened |
| latqcc | -573 | -0.037288 | -0.462177 | improved |
| Q | 0 | +0.000002 | +0.000000 | effectively unchanged |
| RM | 0 | +0.000000 | +0.000000 | unchanged |
| Snow-Water | 0 | +0.000000 | +0.000000 | unchanged |

## Interpretation

Static:
- The residual rain-on-snow release correction moved storage, `Ep`, and `latqcc` in the right direction and reduced the largest storage residual.
- It did not move `RM`, `Q`, or `Snow-Water`, so the next closure work should not chase ET compensation. The remaining high-value target is the downstream WB13/RM publication and baseline snowpack/runoff forcing seam on the material 2014/2016 days.
