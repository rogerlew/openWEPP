# Full 39 Suite Metrics

Status: complete
Evidence mode: Ran

Ran:

- Run root: `/tmp/hphys0294_full_20260605T050323Z`
- Summary: `/tmp/hphys0294_full_20260605T050323Z/reports/hillslope_semantic_summary.md`
- Selected metrics JSON: `/tmp/hphys0294_full_20260605T050323Z/reports/hphys0294_selected_metrics.json`
- Semantic pass: `0/39`

| Symbol | Pass Hillslopes | Total Fail Count | Mean Abs Diff Mean | Max Abs Diff |
| --- | --- | --- | --- | --- |
| Ep | 0/39 | 42688 | 0.633657 | 7.100844 |
| Total-Soil | 0/39 | 52185 | 56.010071 | 317.130129 |
| SoilWaterTotal | 0/39 | 52185 | 56.010071 | 317.130129 |
| Dp | 1/39 | 10961 | 0.050444 | 0.244800 |
| latqcc | 0/39 | 38462 | 0.285882 | 3.023092 |
| Q | 39/39 | 0 | 0.000000 | 0.000000 |
| RM | 0/39 | 7097 | 0.256086 | 27.960000 |
| Snow-Water | 0/39 | 10391 | 2.899431 | 65.506840 |

Interpretation:

- `Q` remains closed across all 39 hillslopes.
- Storage parity remains open across all hillslopes.
- `Dp` residual is small relative to storage (`max 0.244800 mm`).
- `latqcc` residual remains material enough for follow-on accounting but does
  not by itself explain mixed storage residual direction.
