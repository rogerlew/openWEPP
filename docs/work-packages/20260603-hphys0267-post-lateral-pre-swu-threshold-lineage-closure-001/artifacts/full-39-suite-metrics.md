# Full 39 Suite Metrics

Status: completed
Evidence mode: Ran

Ran:

- Root: `/tmp/hphys0267_20260603T162040Z`.
- Runtime status:
  `/tmp/hphys0267_20260603T162040Z/reports/hillslope_batch_status.tsv`.
- Semantic status:
  `/tmp/hphys0267_20260603T162040Z/reports/semantic_status.tsv`.
- Summary:
  `/tmp/hphys0267_20260603T162040Z/reports/hillslope_semantic_summary.md`.
- Runtime status: 39/39 hillslope runs returned `rc=0`.
- Semantic pass: `0/39`.

| Symbol | Pass Hillslopes | Total Fail Count | Mean Abs Diff Mean | Max Abs Diff |
| --- | --- | --- | --- | --- |
| Ep | 0/39 | 56132 | 1.669264 | 7.778863 |
| Total-Soil | 0/39 | 55908 | 149.442866 | 611.813445 |
| SoilWaterTotal | 0/39 | 55908 | 149.442866 | 611.813445 |
| Dp | 0/39 | 35445 | 0.150040 | 0.244800 |
| latqcc | 0/39 | 40340 | 0.675265 | 14.760000 |
| Q | 0/39 | 2986 | 0.925027 | 194.715728 |
| RM | 0/39 | 10678 | 2.301802 | 204.850510 |
| Snow-Water | 0/39 | 24137 | 58.195696 | 562.470000 |

Interpretation: trace-only HPHYS0267 changes do not alter physics. Metrics
remain a continuation baseline, not a closure claim.
