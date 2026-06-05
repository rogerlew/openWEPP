# H1/H7/H39 Trace Evidence

Status: complete
Evidence mode: Ran

## Target Trace Root

Ran:

- Trace root: `/tmp/hphys0290_target_traces_current_20260605T011834Z_postfix`
- Source full-suite runfiles: `/tmp/hphys0290_full_release_current_20260605T011429Z_postfix/runs`
- H1: 24,837 trace rows.
- H7: 24,837 trace rows.
- H39: 24,837 trace rows.
- Trace schema: `openwepp-hphys0245-wb11-wb18-wb19-wb17-evappm-branch-trace-v14`.

## Target Hillslope Metrics

Ran:

- Per-hillslope semantic reports: `/tmp/hphys0290_full_release_current_20260605T011429Z_postfix/reports/semantic_reports/`

| Hillslope | Symbol | Fail Count | Mean Abs Diff | Max Abs Diff |
| --- | --- | ---: | ---: | ---: |
| H1 | Ep | 1240 | 1.019153 | 6.777628 |
| H1 | Total-Soil | 1406 | 88.059717 | 268.172859 |
| H1 | Q | 52 | 0.527302 | 36.520285 |
| H1 | RM | 184 | 0.267551 | 25.340000 |
| H1 | Snow-Water | 263 | 2.949825 | 64.037318 |
| H7 | Ep | 1263 | 0.823460 | 7.184699 |
| H7 | Total-Soil | 1352 | 52.538295 | 307.034663 |
| H7 | Q | 57 | 0.584598 | 36.049136 |
| H7 | RM | 180 | 0.247073 | 24.110000 |
| H7 | Snow-Water | 260 | 2.703498 | 46.330458 |
| H39 | Ep | 930 | 0.306491 | 4.404708 |
| H39 | Total-Soil | 1292 | 29.023345 | 287.740292 |
| H39 | Q | 54 | 0.546601 | 36.673691 |
| H39 | RM | 183 | 0.259613 | 23.790000 |
| H39 | Snow-Water | 269 | 2.989004 | 50.918099 |

## Material Rows

Ran:

| Hillslope | Date | Post-Winter Rain (m) | Routed Melt (m) | Released Rain (m) | WB12 Infiltration (m) | WB13 RM (mm) | Snow-Water (mm) | Total-Soil (mm) | Note |
| --- | --- | ---: | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| H1 | 2014-142 | 0.000000000 | 0.036136740 | 0.000000000 | 0.000717874 | 36.136740 | 88.187978 | 299.276292 | routed melt lineage unchanged |
| H1 | 2014-143 | 0.000000000 | 0.045536138 | 0.001700000 | 0.045536138 | 45.536138 | 44.351840 | 339.969391 | routed melt lineage unchanged |
| H7 | 2014-146 | 0.000000000 | 0.014472955 | 0.002620000 | 0.014472955 | 14.472955 | 0.000000 | 320.946416 | routed melt lineage unchanged |
| H7 | 2016-110 | 0.000000000 | 0.029922681 | 0.000000000 | 0.000770275 | 29.922681 | 5.263813 | 232.222427 | routed melt lineage unchanged |
| H39 | 2014-143 | 0.000000000 | 0.045601941 | 0.001700000 | 0.045601941 | 45.601941 | 39.922453 | 260.556496 | routed melt lineage unchanged |
| H39 | 2014-146 | 0.002620000 | 0.000000000 | 0.000382500 | 0.001782787 | 2.620000 | 0.000000 | 289.366778 | warm-rain restoration branch explicit |

Interpretation: HPHYS0290 proves WB13 now consumes an explicit post-winter rain surface. H39 2014-146 remains `RM=2.62 mm` because the baseline `contin.for` warm-rain/no-snow restoration branch preserves `warain`; the prior suspected publication inference is now an explicit producer surface rather than a WB13 reconstruction.
