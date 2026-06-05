# Full H1..H39 Suite Metrics

Status: complete
Evidence mode: Ran

# HPHYS0289 Full H1..H39 Semantic Summary

Ran:

- Root: `/tmp/hphys0289_full_release_current_20260605T000159Z`
- Runtime status: `/tmp/hphys0289_full_release_current_20260605T000159Z/reports/hillslope_batch_status.tsv`
- Semantic status: `/tmp/hphys0289_full_release_current_20260605T000159Z/reports/semantic_status.tsv`
- Semantic summary: `/tmp/hphys0289_full_release_current_20260605T000159Z/reports/hillslope_semantic_summary.md`
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
| RM | 0/39 | 5868 | 0.258409 | 27.960000 |
| Snow-Water | 0/39 | 10391 | 2.899431 | 65.506840 |
| P | 39/39 | 0 | 0.000000 | 0.000000 |

## Delta From HPHYS0288

Ran:

- HPHYS0288 comparison root: `/tmp/hphys0288_full_release_final_v13_20260604T163204Z`

| Symbol | HPHYS0288 Fail | HPHYS0289 Fail | Fail Count Delta | HPHYS0288 Mean Abs | HPHYS0289 Mean Abs | Mean Abs Diff Delta | Direction |
| --- | ---: | ---: | ---: | ---: | ---: | ---: | --- |
| Ep | 45401 | 45401 | 0 | 0.727061 | 0.727061 | +0.000000 | unchanged |
| Es | 500 | 500 | 0 | 0.010422 | 0.010422 | +0.000000 | unchanged |
| Er | 0 | 0 | 0 | 0.000000 | 0.000000 | +0.000000 | unchanged |
| Total-Soil | 52521 | 52521 | 0 | 57.069194 | 57.069194 | +0.000000 | unchanged |
| SoilWaterTotal | 52521 | 52521 | 0 | 57.069194 | 57.069194 | +0.000000 | unchanged |
| Dp | 9220 | 9220 | 0 | 0.042845 | 0.042845 | +0.000000 | unchanged |
| latqcc | 36003 | 36003 | 0 | 0.373461 | 0.373461 | +0.000000 | unchanged |
| Q | 2108 | 2108 | 0 | 0.552220 | 0.552220 | +0.000000 | unchanged |
| RM | 6633 | 5868 | -765 | 0.248018 | 0.258409 | +0.010391 | mixed |
| Snow-Water | 10391 | 10391 | 0 | 2.899431 | 2.899431 | +0.000000 | unchanged |
| P | 0 | 0 | 0 | 0.000000 | 0.000000 | +0.000000 | unchanged |

## Target Trace Evidence

Ran:

- Trace root: `/tmp/hphys0289_target_traces_current_20260605T000516Z`
- H1/H7/H39 details: `docs/work-packages/20260604-hphys0289-wb13-rm-snowwater-publication-lineage-closure-001/artifacts/h1-h7-h39-trace-evidence.md`

## Interpretation

Static:

- HPHYS0289 moved only WB13 `RM`; fail count improved by 765 rows, confirming the publication lineage changed from SWE-delta proxy to routed `wmelt`.
- `RM` mean absolute residual worsened slightly, and `Q`/`Snow-Water` did not move, so this package remains `executed-hold` rather than closed.
- H39 2014-146 target evidence shows the next package should publish and consume an explicit post-winter `rain(iplane)` surface instead of inferring post-winter rain from raw `prcp` and snow-state activity.
