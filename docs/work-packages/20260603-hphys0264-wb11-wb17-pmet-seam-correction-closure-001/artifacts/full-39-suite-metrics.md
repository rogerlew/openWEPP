# Full H1..H39 Suite Metrics

Status: completed

Evidence mode: Ran

Ran:

- Command:
  `/workdir/wepppy/.venv/bin/python docs/work-packages/20260602-hphys0254-wb11-initial-storage-projection-closure-001/artifacts/hphys0254_diagnostics.py --run-root /tmp/hphys0264_20260603T083941Z`.
- Summary:
  `/tmp/hphys0264_20260603T083941Z/reports/hillslope_semantic_summary.md`.
- Semantic pass: `0/39`.

| Symbol | Pass Hillslopes | Total Fail Count | Mean Abs Diff Mean | Max Abs Diff |
| --- | --- | --- | --- | --- |
| Ep | 0/39 | 56132 | 1.669264 | 7.778863 |
| Es | 38/39 | 1188 | 0.018815 | 1.890000 |
| Er | 39/39 | 0 | 0.000000 | 0.000000 |
| Total-Soil | 0/39 | 55908 | 149.442866 | 611.813445 |
| SoilWaterTotal | 0/39 | 55908 | 149.442866 | 611.813445 |
| Dp | 0/39 | 35445 | 0.150040 | 0.244800 |
| latqcc | 0/39 | 40340 | 0.675265 | 14.760000 |
| Q | 0/39 | 2986 | 0.925027 | 194.715728 |
| RM | 0/39 | 10678 | 2.301802 | 204.850510 |
| Snow-Water | 0/39 | 24137 | 58.195696 | 562.470000 |

Targeted day-1 continuation signals:

| Hillslope | Total-Soil diff mm | Dp diff mm | latqcc diff mm | Ep diff mm |
| --- | --- | --- | --- | --- |
| H1 | -0.015135 | 0.004798 | 0.023532 | 0.001823 |
| H7 | 0.023570 | 0.004800 | 0.047995 | 0.001823 |
| H39 | -0.103459 | 0.004800 | 0.180364 | 0.001823 |

Interpretation:

- The PMET seam correction improves the targeted seam invariant, but full
  H1..H39 semantic parity remains a program `HOLD`.
- Dominant residual families remain aggregate storage, snow/runoff timing,
  lateral-flow, percolation, and longer-season `Ep`.
