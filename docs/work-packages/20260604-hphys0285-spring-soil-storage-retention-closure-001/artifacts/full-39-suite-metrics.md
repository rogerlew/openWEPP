# Full H1..H39 Suite Metrics

Status: complete
Evidence mode: Ran

## Run Artifacts

Ran:
- Candidate run root: `/tmp/hphys0285_full_release_final_20260604T201242Z`
- Runtime status: `/tmp/hphys0285_full_release_final_20260604T201242Z/reports/hillslope_batch_status.tsv`
- Semantic status: `/tmp/hphys0285_full_release_final_20260604T201242Z/reports/semantic_status.tsv`
- Semantic summary: `/tmp/hphys0285_full_release_final_20260604T201242Z/reports/hillslope_semantic_summary.md`
- Semantic summary JSON: `/tmp/hphys0285_full_release_final_20260604T201242Z/reports/hillslope_semantic_summary.json`

## Runtime And Semantic Status

Ran:
- Runtime completed: `39/39`.
- Semantic reports completed: `39/39`.
- Semantic pass: `0/39`.

## Selected Metrics

Ran:

| Symbol | Pass Hillslopes | Total Fail Count | Mean Abs Diff Mean | Max Abs Diff |
| --- | ---: | ---: | ---: | ---: |
| Ep | 0/39 | 46711 | 0.759616 | 7.334713 |
| Es | 38/39 | 517 | 0.010479 | 1.825405 |
| Er | 39/39 | 0 | 0.000000 | 0.000000 |
| Total-Soil | 0/39 | 54888 | 71.751081 | 350.397536 |
| SoilWaterTotal | 0/39 | 54888 | 71.751081 | 350.397536 |
| Dp | 1/39 | 9499 | 0.043905 | 0.244800 |
| latqcc | 0/39 | 41730 | 0.476975 | 12.327253 |
| Q | 0/39 | 2108 | 0.552218 | 38.472185 |
| RM | 0/39 | 6633 | 0.248018 | 27.960000 |
| Snow-Water | 0/39 | 10391 | 2.899431 | 65.506840 |
| P | 39/39 | 0 | 0.000000 | 0.000000 |

## HPHYS0284 To HPHYS0285 Delta

Ran:

| Symbol | HPHYS0284 Mean Abs | HPHYS0285 Mean Abs | Mean Delta | Fail Delta | Max Delta |
| --- | ---: | ---: | ---: | ---: | ---: |
| Ep | 1.145444 | 0.759616 | -0.385828 | -7349 | -0.212411 |
| Total-Soil | 89.531529 | 71.751081 | -17.780448 | -814 | -24.539589 |
| SoilWaterTotal | 89.531529 | 71.751081 | -17.780448 | -814 | -24.539589 |
| Dp | 0.078495 | 0.043905 | -0.034590 | -8447 | -0.000000 |
| latqcc | 0.555122 | 0.476975 | -0.078147 | -1814 | 0.139973 |
| Q | 0.552218 | 0.552218 | 0.000000 | 0 | 0.000000 |
| RM | 0.248018 | 0.248018 | 0.000000 | 0 | -0.000000 |
| Snow-Water | 2.899431 | 2.899431 | 0.000000 | 0 | -0.000000 |

## Continuation Signal

Static + Ran:
- HPHYS0285 materially improved storage, Dp, latqcc, and Ep residuals, but did not close semantic parity.
- H7/H39 spring 2014 remain too dry during meltout (`Total-Soil` max deltas near Julian 145-146, candidate roughly 278-289 mm vs baseline roughly 580-612 mm).
- H1 2015 late season is too wet (`Total-Soil` candidate roughly 340 mm vs baseline roughly 35-38 mm near Julian 222-225).
- `Q`, `RM`, and `Snow-Water` are unchanged from HPHYS0284, so the next package should focus below the melt/runoff partition: layer-capacity/retention/percolation publication and WB17 withdrawal from corrected storage, not another snow timing change.
