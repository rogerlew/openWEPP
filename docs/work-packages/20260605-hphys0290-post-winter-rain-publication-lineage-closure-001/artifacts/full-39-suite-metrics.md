# Full H1..H39 Suite Metrics

Status: complete
Evidence mode: Ran

# HPHYS0290 Full H1..H39 Semantic Summary

Ran:

- Root: `/tmp/hphys0290_full_release_current_20260605T011429Z_postfix`
- Runtime status: `/tmp/hphys0290_full_release_current_20260605T011429Z_postfix/reports/hillslope_batch_status.tsv`
- Semantic status: `/tmp/hphys0290_full_release_current_20260605T011429Z_postfix/reports/semantic_status.tsv`
- Semantic summary: `/tmp/hphys0290_full_release_current_20260605T011429Z_postfix/reports/hillslope_semantic_summary.md`
- Runtime pass: `39/39`
- Semantic pass: `0/39`

| Symbol | Pass Hillslopes | Total Fail Count | Mean Abs Diff Mean | Max Abs Diff |
| --- | --- | ---: | ---: | ---: |
| Ep | 0/39 | 45401 | 0.727061 | 7.242659 |
| Es | 38/39 | 500 | 0.010422 | 1.825681 |
| Er | 39/39 | 0 | 0.000000 | 0.000000 |
| Total-Soil | 0/39 | 52521 | 57.069194 | 348.886998 |
| SoilWaterTotal | 0/39 | 52521 | 57.069194 | 348.886998 |
| Dp | 1/39 | 9220 | 0.042845 | 0.244800 |
| latqcc | 0/39 | 36003 | 0.373461 | 11.865076 |
| Q | 0/39 | 2108 | 0.552220 | 38.472185 |
| RM | 0/39 | 7097 | 0.256086 | 27.960000 |
| Snow-Water | 0/39 | 10391 | 2.899431 | 65.506840 |
| P | 39/39 | 0 | 0.000000 | 0.000000 |

## Delta From HPHYS0289

Ran:

- HPHYS0289 comparison root: `/tmp/hphys0289_full_release_current_20260605T000159Z`

| Symbol | HPHYS0289 Fail | HPHYS0290 Fail | Fail Count Delta | HPHYS0289 Mean Abs | HPHYS0290 Mean Abs | Mean Abs Diff Delta | Direction |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| Ep | 45401 | 45401 | +0 | 0.727061 | 0.727061 | +0.000000 | unchanged |
| Es | 500 | 500 | +0 | 0.010422 | 0.010422 | +0.000000 | unchanged |
| Er | 0 | 0 | +0 | 0.000000 | 0.000000 | +0.000000 | unchanged |
| Total-Soil | 52521 | 52521 | +0 | 57.069194 | 57.069194 | +0.000000 | unchanged |
| SoilWaterTotal | 52521 | 52521 | +0 | 57.069194 | 57.069194 | +0.000000 | unchanged |
| Dp | 9220 | 9220 | +0 | 0.042845 | 0.042845 | +0.000000 | unchanged |
| latqcc | 36003 | 36003 | +0 | 0.373461 | 0.373461 | +0.000000 | unchanged |
| Q | 2108 | 2108 | +0 | 0.552220 | 0.552220 | +0.000000 | unchanged |
| RM | 5868 | 7097 | +1229 | 0.258409 | 0.256086 | -0.002324 | mixed |
| Snow-Water | 10391 | 10391 | +0 | 2.899431 | 2.899431 | +0.000000 | unchanged |
| P | 0 | 0 | +0 | 0.000000 | 0.000000 | +0.000000 | unchanged |

## Interpretation

Static:

- HPHYS0290 made the post-winter `rain(iplane)` equivalent explicit and fail-closed, but it did not close semantic parity; suite pass remains `0/39`.
- `RM` mean absolute residual improved slightly (`-0.002324 mm`), while `RM` fail count increased (`+1229` rows). This is a mixed publication result, not a closure result.
- `Ep`, `Total-Soil`, `SoilWaterTotal`, `Dp`, `latqcc`, `Q`, and `Snow-Water` did not move, keeping the continuation focus upstream of WB13 publication on snowpack state/timing and storage/runoff partition lineage.
