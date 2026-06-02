# Full 39 Hillslope Suite Metrics

Status: complete

Evidence mode: ran

Ran:

- Run root: `/tmp/hphys0254_20260602T220046Z`
- Runtime status: `/tmp/hphys0254_20260602T220046Z/reports/hillslope_batch_status.tsv`
- Semantic status: `/tmp/hphys0254_20260602T220046Z/reports/semantic_status.tsv`
- Semantic summary: `/tmp/hphys0254_20260602T220046Z/reports/hillslope_semantic_summary.md`

Runtime:

- `39/39` hillslope runs returned rc `0`.
- All semantic comparisons had `1461` common rows.
- Semantic pass remains `0/39`.

Semantic residual summary:

| Symbol | Pass Hillslopes | Total Fail Count | Mean Abs Diff Mean | Max Abs Diff |
| --- | --- | --- | --- | --- |
| Ep | 0/39 | 56391 | 1.700230 | 7.779928 |
| Total-Soil | 0/39 | 56941 | 167.165068 | 618.513538 |
| SoilWaterTotal | 0/39 | 56941 | 167.165068 | 618.513538 |
| Dp | 0/39 | 41028 | 0.172796 | 0.240000 |
| latqcc | 0/39 | 39871 | 0.805148 | 28.005815 |
| Q | 0/39 | 2986 | 0.925027 | 194.715728 |
| RM | 0/39 | 10678 | 2.301802 | 204.850510 |
| Snow-Water | 0/39 | 24137 | 58.195696 | 562.470000 |

Interpretation:

- HPHYS0254 closes the seed-depth lineage defect but does not close full-suite semantic parity.
- Remaining continuation focus should stay on process residuals visible after seed closure: WB19 `latqcc`, snow/runoff timing, and Ep/plant-water coupling.
