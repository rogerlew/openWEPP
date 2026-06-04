# Full H1..H39 Suite Metrics

Status: complete
Evidence mode: Ran

## Ran: Final Suite

- Run root: `/tmp/hphys0283_full3_20260604T163035Z`.
- Runtime status: `/tmp/hphys0283_full3_20260604T163035Z/reports/hillslope_batch_status.tsv`.
- Semantic status: `/tmp/hphys0283_full3_20260604T163035Z/reports/semantic_status.tsv`.
- Semantic summary: `/tmp/hphys0283_full3_20260604T163035Z/reports/hillslope_semantic_summary.md`.
- Semantic pass: `0/39`.

| Symbol | Pass Hillslopes | Total Fail Count | Mean Abs Diff Mean | Max Abs Diff |
| --- | --- | ---: | ---: | ---: |
| Ep | 0/39 | 53875 | 1.120605 | 7.547063 |
| Total-Soil | 0/39 | 55728 | 83.841688 | 364.212554 |
| SoilWaterTotal | 0/39 | 55728 | 83.841688 | 364.212554 |
| Dp | 1/39 | 16932 | 0.074413 | 0.244800 |
| latqcc | 0/39 | 44137 | 0.550372 | 12.149105 |
| Q | 0/39 | 2388 | 0.672385 | 40.914615 |
| RM | 0/39 | 7349 | 0.324492 | 41.480927 |
| Snow-Water | 0/39 | 13799 | 4.909469 | 102.625114 |

## Ran: Movement From Post-0281 Baseline

| Symbol | Post-0281 Mean Abs Diff | HPHYS0283 Mean Abs Diff | Direction |
| --- | ---: | ---: | --- |
| Ep | 1.669264 | 1.120605 | improved |
| Total-Soil | 149.442866 | 83.841688 | improved |
| SoilWaterTotal | 149.442866 | 83.841688 | improved |
| Dp | 0.150040 | 0.074413 | improved |
| latqcc | 0.675265 | 0.550372 | improved |
| Q | 1.245240 | 0.672385 | improved |
| RM | 0.324492 | 0.324492 | unchanged |
| Snow-Water | 4.909469 | 4.909469 | unchanged |

## Ran: Spring 2014 Collapse Rows

| Case | Baseline Total-Soil | Candidate Before | Candidate After | Candidate Q After | Candidate Snow-Water After |
| --- | ---: | ---: | ---: | ---: | ---: |
| H1 Julian 145 | 645.560 | 33.747 | 343.986 | 0.000 | 61.263 |
| H7 Julian 146 | 611.940 | 31.793 | 296.668 | 0.000 | 48.472 |
| H39 Julian 145 | 580.470 | 45.485 | 303.333 | 0.000 | 59.304 |

## Interpretation

- The observed spring storage collapse to roughly `30..45 mm` is resolved for the targeted rows by same-pass melt ingress.
- Remaining `Total-Soil` residual is still material and now localizes to earlier snowpack timing/retention plus remaining runoff/soil-storage divergence, not the original melt-only runoff bypass.
