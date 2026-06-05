# H1/H7/H39 Trace Evidence

Status: complete
Evidence mode: Ran

## Target Trace Root

Ran:

- Trace root: `/tmp/hphys0289_target_traces_current_20260605T000516Z`
- Source full-suite runfiles: `/tmp/hphys0289_full_release_current_20260605T000159Z/runs`
- H1: 24,837 trace rows.
- H7: 24,837 trace rows.
- H39: 24,837 trace rows.
- Trace schema: `openwepp-hphys0245-wb11-wb18-wb19-wb17-evappm-branch-trace-v13`.

## Target Hillslope Metrics

Ran:

- Per-hillslope semantic reports: `/tmp/hphys0289_full_release_current_20260605T000159Z/reports/semantic_reports/`

| Hillslope | Symbol | Fail Count | Mean Abs Diff | Max Abs Diff |
| --- | --- | ---: | ---: | ---: |
| H1 | Ep | 1240 | 1.019153 | 6.777628 |
| H1 | Total-Soil | 1406 | 88.059717 | 268.172859 |
| H1 | Q | 52 | 0.527302 | 36.520285 |
| H1 | RM | 152 | 0.269194 | 25.340000 |
| H1 | Snow-Water | 263 | 2.949825 | 64.037318 |
| H7 | Ep | 1263 | 0.823460 | 7.184699 |
| H7 | Total-Soil | 1352 | 52.538295 | 307.034663 |
| H7 | Q | 57 | 0.584598 | 36.049136 |
| H7 | RM | 147 | 0.248589 | 24.110000 |
| H7 | Snow-Water | 260 | 2.703498 | 46.330458 |
| H39 | Ep | 930 | 0.306491 | 4.404708 |
| H39 | Total-Soil | 1292 | 29.023345 | 287.740292 |
| H39 | Q | 54 | 0.546601 | 36.673691 |
| H39 | RM | 151 | 0.262013 | 23.790000 |
| H39 | Snow-Water | 269 | 2.989004 | 50.918099 |

## Material Rows

Ran:

| Hillslope | Date | Released Rain (m) | Routed Melt (m) | WB12 Infiltration (m) | WB13 RM (mm) | Snow-Water (mm) | Total-Soil (mm) | Note |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| H1 | 2014-142 | 0.000000 | 0.036136740 | 0.000717874 | 36.136740 | 88.187978 | 299.276292 | high melt, limited infiltration |
| H1 | 2014-143 | 0.001700 | 0.045536138 | 0.045536138 | 45.536138 | 44.351840 | 339.969391 | released-rain seam active |
| H7 | 2014-146 | 0.002620 | 0.014472955 | 0.014472955 | 14.472955 | 0.000000 | 320.946416 | released-rain seam active |
| H7 | 2016-110 | 0.000000 | 0.029922681 | 0.000770275 | 29.922681 | 5.263813 | 232.222427 | high melt, limited infiltration |
| H39 | 2014-143 | 0.001700 | 0.045601941 | 0.045601941 | 45.601941 | 39.922453 | 260.556496 | released-rain seam active |
| H39 | 2014-146 | 0.000382500 | 0.000000 | 0.001782787 | 2.620000 | 0.000000 | 289.366778 | remaining RM residual: snow-free raw rain branch still inferred |

Interpretation: H1/H7/H39 prove routed melt now reaches WB13 `RM`. H39 2014-146 remains diagnostic for the next package because WB13 still infers post-winter rain from raw `prcp` when runtime SWE/routed melt are zero; baseline has an explicit post-winter `rain(iplane)` value.
