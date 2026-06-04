# H1 H7 H39 Trace Evidence

Status: complete
Evidence mode: Ran

## Target Hillslope Metrics

Ran:

- Per-hillslope semantic reports: `/tmp/hphys0288_full_release_final_v13_20260604T163204Z/reports/semantic_reports/`

| Hillslope | Symbol | Fail Count | Mean Abs Diff | Max Abs Diff |
| --- | --- | ---: | ---: | ---: |
| H1 | Ep | 1240 | 1.019153 | 6.777628 |
| H1 | Total-Soil | 1406 | 88.059717 | 268.172859 |
| H1 | Q | 52 | 0.527302 | 36.520285 |
| H1 | RM | 170 | 0.258521 | 25.340000 |
| H1 | Snow-Water | 263 | 2.949825 | 64.037318 |
| H7 | Ep | 1263 | 0.823460 | 7.184699 |
| H7 | Total-Soil | 1352 | 52.538295 | 307.034663 |
| H7 | Q | 57 | 0.584598 | 36.049136 |
| H7 | RM | 168 | 0.238843 | 24.110000 |
| H7 | Snow-Water | 260 | 2.703498 | 46.330458 |
| H39 | Ep | 930 | 0.306491 | 4.404708 |
| H39 | Total-Soil | 1292 | 29.023345 | 287.740292 |
| H39 | Q | 54 | 0.546601 | 36.673691 |
| H39 | RM | 168 | 0.251790 | 23.790000 |
| H39 | Snow-Water | 269 | 2.989004 | 50.918099 |

## Target Trace Root

Ran:

- Trace root: `/tmp/hphys0288_target_traces_v13_20260604T162402Z`
- H1: 24,837 trace rows; 409 post-WB13 released-rain rows.
- H7: 24,837 trace rows; 402 post-WB13 released-rain rows.
- H39: 24,837 trace rows; 409 post-WB13 released-rain rows.
- Trace schema: `openwepp-hphys0245-wb11-wb18-wb19-wb17-evappm-branch-trace-v13`.

## Representative Released-Rain Rows

Ran:

| Hillslope | Date | Released Rain (m) | Routed Melt (m) | WB12 Infiltration (m) | WB18 Theta Sum (m) | WB13 RM (mm) | Q (m) | Snow-Water (mm) | Total-Soil (mm) |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | ---: |
| H1 | 2013-005 | 0.000176667 | 0.000176667 | 0.000176667 | 0.308920732 | 0.176667 | 0.000228947 | 3.113333 | 340.322343 |
| H7 | 2013-005 | 0.000176667 | 0.000176667 | 0.000176667 | 0.252348142 | 0.176667 | 0.000509254 | 3.113333 | 284.138279 |
| H39 | 2013-005 | 0.000176667 | 0.000176667 | 0.000176667 | 0.333838624 | 0.176667 | 0.002609543 | 3.113333 | 376.249525 |

## Material Spring Residual Rows

Ran:

| Hillslope | Date | Released Rain (m) | Routed Melt (m) | WB12 Infiltration (m) | WB13 RM (mm) | Snow-Water (mm) | Total-Soil (mm) | Note |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| H1 | 2014-142 | 0.000000 | 0.036136740 | 0.000717874 | 36.136740 | 88.187978 | 299.276292 | top H1 row has melt residual, not released-rain residual |
| H1 | 2014-143 | 0.001700 | 0.045536138 | 0.045536138 | 45.536138 | 44.351840 | 339.969391 | released-rain seam active |
| H7 | 2014-146 | 0.002620 | 0.014472955 | 0.014472955 | 14.472955 | 0.000000 | 320.946416 | released-rain seam active |
| H7 | 2016-110 | 0.000000 | 0.029922681 | 0.000770275 | 29.922681 | 5.263813 | 232.222427 | top H7 row has melt residual, not released-rain residual |
| H39 | 2014-143 | 0.001700 | 0.045601941 | 0.045601941 | 45.601941 | 39.922453 | 260.556496 | released-rain seam active |
| H39 | 2014-146 | 0.000382500 | 0.000000 | 0.001782787 | 2.620000 | 0.000000 | 289.366778 | released-rain present but no routed melt remains after snowpack depletion |

## Trace Closure Evidence

Ran:

- `cargo test -p openwepp-runner hphys0288_trace_row_captures_rain_on_snow_release_without_snowpack_loss -- --nocapture`
- The synthetic closure vector serializes `snow_hourly_rain_released_sum_m = 0.002`, `snow_hourly_melt_sum_m = 0.002`, and `snow_runtime_swe_closure_error_m = 0.0`.
