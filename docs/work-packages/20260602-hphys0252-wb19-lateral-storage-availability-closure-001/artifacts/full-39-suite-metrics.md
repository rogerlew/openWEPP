# Full 39 Hillslope Suite Metrics

Status: complete

Evidence mode: ran

Ran:

- Runtime root: `/tmp/hphys0252_20260602T195147Z`.
- Runtime status file:
  `/tmp/hphys0252_20260602T195147Z/reports/hillslope_batch_status.tsv`.
- Semantic status file:
  `/tmp/hphys0252_20260602T195147Z/reports/semantic_status.tsv`.
- Semantic summary:
  `/tmp/hphys0252_20260602T195147Z/reports/hillslope_semantic_summary.md`.
- Apples-to-apples delta:
  `/tmp/hphys0252_20260602T195147Z/reports/hphys0252_apples_to_apples_delta_from_hphys0251.md`.

Summary:

- Runtime success: `39/39`.
- Semantic reports: `39/39`.
- Semantic pass: `0/39`.

| Symbol | Pass Hillslopes | Total Fail Count | Mean Abs Diff Mean | Max Abs Diff |
|---|---:|---:|---:|---:|
| `Ep` | 0/39 | 55942 | 1.70276 | 7.77999 |
| `Total-Soil` | 0/39 | 56979 | 170.349 | 620.616 |
| `SoilWaterTotal` | 0/39 | 56979 | 170.349 | 620.616 |
| `Dp` | 0/39 | 40763 | 0.172318 | 0.24 |
| `latqcc` | 0/39 | 39839 | 0.802705 | 36.7496 |
| `Q` | 0/39 | 2980 | 0.925027 | 194.716 |
| `RM` | 0/39 | 10678 | 2.3018 | 204.851 |
| `Snow-Water` | 0/39 | 24141 | 58.1957 | 562.47 |

Apples-to-apples delta from HPHYS0251:

- Selected-symbol fail count delta is zero for `Ep`, `Total-Soil`,
  `SoilWaterTotal`, `Dp`, `Es`, `Q`, `RM`, `Snow-Water`, and `latqcc`.
- HPHYS0252 remains a correctness fix, not a residual-reduction closure.
