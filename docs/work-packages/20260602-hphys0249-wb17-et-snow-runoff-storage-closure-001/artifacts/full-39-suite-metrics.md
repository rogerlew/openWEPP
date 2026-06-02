# Full 39 Hillslope Suite Metrics

Status: complete

Evidence mode: ran

Ran:

- Root: `/tmp/hphys0249_20260602T161254Z_postreview`
- Runtime status:
  `/tmp/hphys0249_20260602T161254Z_postreview/reports/hillslope_batch_status.tsv`
- Semantic status:
  `/tmp/hphys0249_20260602T161254Z_postreview/reports/semantic_status.tsv`
- Semantic summary:
  `/tmp/hphys0249_20260602T161254Z_postreview/reports/hillslope_semantic_summary.json`
- Runtime success: `39/39`.
- Semantic report success: `39/39`.
- Semantic pass: `0/39`.
- Common rows: `1461..1461`.

| Column | Pass | Fail Count Sum | Mean Abs Mean | Mean Abs Max | Max Abs Max | Worst H |
|---|---:|---:|---:|---:|---:|---:|
| `SoilWaterTotal` | 0/39 | 56898 | 131.293228 | 213.271419 | 565.718633 | H24 |
| `Total-Soil` | 0/39 | 56898 | 131.293228 | 213.271419 | 565.718633 | H24 |
| `Ep` | 0/39 | 56834 | 1.739422 | 2.126851 | 7.780000 | H1 |
| `Dp` | 0/39 | 40116 | 0.169502 | 0.220824 | 0.240000 | H1 |
| `latqcc` | 0/39 | 39813 | 0.803162 | 1.403049 | 36.775073 | H33 |
| `Snow-Water` | 0/39 | 24137 | 58.195696 | 65.985290 | 562.470000 | H9 |
| `RM` | 0/39 | 10678 | 2.301802 | 2.401303 | 204.850510 | H6 |
| `Es` | 0/39 | 3272 | 0.036841 | 0.391494 | 1.890000 | H6 |
| `Q` | 0/39 | 2986 | 0.925027 | 1.117244 | 194.715728 | H6 |
| `ProfileFCStore` | 12/39 | 39447 | 2.052691 | 9.334426 | 9.334426 | H7 |
| `ProfileWPStore` | 38/39 | 1461 | 0.057297 | 1.669863 | 1.669863 | H7 |

Continuation reading:

1. `Es` is no longer the dominant WB17 residual after layer-first soil
   evaporation correction.
2. `Ep` is unchanged and remains the highest-priority WB17 residual.
3. Snow/runoff timing columns are unchanged and require a separate package.
4. Aggregate storage remains worse than pre-HPHYS0249 after removing scalar
   over-withdrawal; reassess after `Ep` and snow/runoff timing closure.
