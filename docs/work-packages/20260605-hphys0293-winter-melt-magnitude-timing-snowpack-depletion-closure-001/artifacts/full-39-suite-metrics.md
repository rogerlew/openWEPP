# Full H1..H39 Suite Metrics

Status: complete
Evidence mode: Ran

Ran:

- Command: `.venv/bin/python docs/work-packages/20260605-hphys0293-winter-melt-magnitude-timing-snowpack-depletion-closure-001/artifacts/hphys0293_diagnostics.py --run-root /tmp/hphys0293_full_20260604T212429Z --trace-max-days 1800`
- Runtime status: `/tmp/hphys0293_full_20260604T212429Z/reports/hillslope_batch_status.tsv`
- Semantic status: `/tmp/hphys0293_full_20260604T212429Z/reports/semantic_status.tsv`
- Selected metrics JSON: `/tmp/hphys0293_full_20260604T212429Z/reports/hphys0293_selected_metrics.json`

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

- Full-suite semantic parity remains `0/39`.
- `Q` parity remains closed at `39/39`, preserving the HPHYS0292 WB14 capacity result.
- `RM` and `Snow-Water` remain open and are classified by HPHYS0293 as snow-producer depletion/timing residuals, not WB14 residual `Q`.
