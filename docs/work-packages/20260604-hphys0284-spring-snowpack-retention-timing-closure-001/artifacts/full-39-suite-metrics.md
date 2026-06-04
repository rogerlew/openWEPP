# Full H1..H39 Suite Metrics

Status: complete
Evidence mode: Ran

## Ran: Final Suite

- Run root: `/tmp/hphys0284_full_release_20260604T182144Z`.
- Runtime status: `/tmp/hphys0284_full_release_20260604T182144Z/reports/hillslope_batch_status.tsv`.
- Semantic status: `/tmp/hphys0284_full_release_20260604T182144Z/reports/semantic_status.tsv`.
- Semantic summary: `/tmp/hphys0284_full_release_20260604T182144Z/reports/hillslope_semantic_summary.md`.
- Runtime completed: `39/39`.
- Semantic reports completed: `39/39`.
- Semantic pass: `0/39`.

| Symbol | Pass Hillslopes | Total Fail Count | Mean Abs Diff Mean | Max Abs Diff |
| --- | --- | ---: | ---: | ---: |
| Ep | 0/39 | 54060 | 1.145444 | 7.547124 |
| Total-Soil | 0/39 | 55702 | 89.531529 | 374.937125 |
| SoilWaterTotal | 0/39 | 55702 | 89.531529 | 374.937125 |
| Dp | 0/39 | 17946 | 0.078495 | 0.244800 |
| latqcc | 0/39 | 43544 | 0.555122 | 12.187280 |
| Q | 0/39 | 2108 | 0.552218 | 38.472185 |
| RM | 0/39 | 6633 | 0.248018 | 27.960000 |
| Snow-Water | 0/39 | 10391 | 2.899431 | 65.506840 |

## Ran: Movement From HPHYS0283

| Symbol | HPHYS0283 Mean Abs Diff | HPHYS0284 Mean Abs Diff | Direction |
| --- | ---: | ---: | --- |
| Ep | 1.120605 | 1.145444 | worsened |
| Total-Soil | 83.841688 | 89.531529 | worsened |
| SoilWaterTotal | 83.841688 | 89.531529 | worsened |
| Dp | 0.074413 | 0.078495 | worsened |
| latqcc | 0.550372 | 0.555122 | worsened |
| Q | 0.672385 | 0.552218 | improved |
| RM | 0.324492 | 0.248018 | improved |
| Snow-Water | 4.909469 | 2.899431 | improved |

## Interpretation

- The corrected negative-melt carry-state fix materially improves `Snow-Water`, `RM`, and `Q`.
- The same fix exposes a lower post-meltout soil-storage state; `Total-Soil` and downstream `Ep`/`Dp` residuals remain open and should not be assigned back to snow timing.
