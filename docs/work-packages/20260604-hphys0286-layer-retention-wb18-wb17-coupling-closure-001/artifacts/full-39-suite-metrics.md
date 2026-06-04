# Full H1..H39 Suite Metrics

Status: complete
Evidence mode: Ran

## Run Artifacts

Ran:
- Candidate run root: `/tmp/hphys0286_full_release_20260604T211814Z`
- Runtime status: `/tmp/hphys0286_full_release_20260604T211814Z/reports/hillslope_batch_status.tsv`
- Semantic status: `/tmp/hphys0286_full_release_20260604T211814Z/reports/semantic_status.tsv`
- Semantic summary: `/tmp/hphys0286_full_release_20260604T211814Z/reports/hillslope_semantic_summary.md`
- Semantic summary JSON: `/tmp/hphys0286_full_release_20260604T211814Z/reports/hillslope_semantic_summary.json`

## Runtime And Semantic Status

Ran:
- Runtime completed: `39/39`.
- Semantic reports completed: `39/39`.
- Semantic pass: `0/39`.

## Selected Metrics

Ran:

| Symbol | Pass Hillslopes | Total Fail Count | Mean Abs Diff Mean | Max Abs Diff |
| --- | ---: | ---: | ---: | ---: |
| Ep | 0/39 | 46375 | 0.742663 | 7.328460 |
| Es | 38/39 | 518 | 0.010479 | 1.825501 |
| Er | 39/39 | 0 | 0.000000 | 0.000000 |
| Total-Soil | 0/39 | 54884 | 61.115200 | 365.018266 |
| SoilWaterTotal | 0/39 | 54884 | 61.115200 | 365.018266 |
| Dp | 1/39 | 9104 | 0.042128 | 0.244800 |
| latqcc | 0/39 | 36576 | 0.410749 | 12.327253 |
| Q | 0/39 | 2108 | 0.552218 | 38.472185 |
| RM | 0/39 | 6633 | 0.248018 | 27.960000 |
| Snow-Water | 0/39 | 10391 | 2.899431 | 65.506840 |
| P | 39/39 | 0 | 0.000000 | 0.000000 |

## HPHYS0285 To HPHYS0286 Delta

Ran:

| Symbol | HPHYS0285 Mean Abs | HPHYS0286 Mean Abs | Mean Delta | Fail Delta | Max Delta |
| --- | ---: | ---: | ---: | ---: | ---: |
| Ep | 0.759616 | 0.742663 | -0.016953 | -336 | -0.006252 |
| Es | 0.010479 | 0.010479 | 0.000001 | 1 | 0.000096 |
| Total-Soil | 71.751081 | 61.115200 | -10.635880 | -4 | 14.620730 |
| SoilWaterTotal | 71.751081 | 61.115200 | -10.635880 | -4 | 14.620730 |
| Dp | 0.043905 | 0.042128 | -0.001777 | -395 | 0.000000 |
| latqcc | 0.476975 | 0.410749 | -0.066225 | -5154 | -0.000000 |
| Q | 0.552218 | 0.552218 | 0.000000 | 0 | 0.000000 |
| RM | 0.248018 | 0.248018 | 0.000000 | 0 | -0.000000 |
| Snow-Water | 2.899431 | 2.899431 | 0.000000 | 0 | -0.000000 |
| P | 0.000000 | 0.000000 | 0.000000 | 0 | 0.000000 |

## Continuation Signal

Static + Ran:
- HPHYS0286 materially improved aggregate `Total-Soil`, `SoilWaterTotal`, `latqcc`, `Dp`, and `Ep` residuals.
- `Q`, `RM`, and `Snow-Water` are unchanged, so remaining early-season storage collapse is not closed by post-ingress layer redistribution alone.
- H1/H7/H39 still show large spring 2014 and spring 2016 low-storage residuals after high candidate runoff days, indicating the next package should re-enter snow liquid retention/runoff/infiltration partition authority before further WB17 tuning.
