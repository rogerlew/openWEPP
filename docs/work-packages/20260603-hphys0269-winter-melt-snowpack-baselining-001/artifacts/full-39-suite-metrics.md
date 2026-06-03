# Full 39-Hillslope Suite Metrics

Status: completed/HOLD
Evidence mode: ran

Ran:

- Command: `.venv/bin/python docs/work-packages/20260603-hphys0269-winter-melt-snowpack-baselining-001/artifacts/hphys0269_diagnostics.py --run-root /tmp/hphys0269_full_final_20260603T185740Z --trace-max-days 180`
- Run root: `/tmp/hphys0269_full_final_20260603T185740Z`.
- Build status: `/tmp/hphys0269_full_final_20260603T185740Z/reports/build_status.tsv`.
- Runtime status: `/tmp/hphys0269_full_final_20260603T185740Z/reports/hillslope_batch_status.tsv`.
- Semantic status: `/tmp/hphys0269_full_final_20260603T185740Z/reports/semantic_status.tsv`.
- Semantic summary: `/tmp/hphys0269_full_final_20260603T185740Z/reports/hillslope_semantic_summary.md`.
- Semantic pass: `0/39`.

| Symbol | Pass Hillslopes | Total Fail Count | Mean Abs Diff Mean | Max Abs Diff |
|---|---:|---:|---:|---:|
| Ep | 0/39 | 56132 | 1.669264 | 7.778863 |
| Total-Soil | 0/39 | 55908 | 149.442866 | 611.813445 |
| SoilWaterTotal | 0/39 | 55908 | 149.442866 | 611.813445 |
| Dp | 0/39 | 35445 | 0.150040 | 0.244800 |
| latqcc | 0/39 | 40340 | 0.675265 | 14.760000 |
| Q | 0/39 | 4480 | 0.979774 | 193.834417 |
| RM | 0/39 | 10367 | 2.272853 | 203.969200 |
| Snow-Water | 0/39 | 23976 | 56.627822 | 560.770686 |

Static: the HPHYS0269 slice reduced full-suite `RM` mean absolute diff mean
from HPHYS0268 `2.301802` to `2.272853` and reduced `Snow-Water` mean absolute
diff mean from `58.195696` to `56.627822`. `Q` fail count increased relative
to HPHYS0268, so runoff timing remains coupled to unresolved snowpack lineage.

