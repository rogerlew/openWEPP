# Full 39 Suite Metrics

Status: completed/HOLD
Evidence mode: ran

Ran:

- Command: `.venv/bin/python docs/work-packages/20260603-hphys0270-winter-daily-snowpack-state-closure-001/artifacts/hphys0270_diagnostics.py --run-root /tmp/hphys0270_full_20260603T201051Z --trace-max-days 180`
- Run root: `/tmp/hphys0270_full_20260603T201051Z`.
- Runtime status: `/tmp/hphys0270_full_20260603T201051Z/reports/hillslope_batch_status.tsv`.
- Semantic status: `/tmp/hphys0270_full_20260603T201051Z/reports/semantic_status.tsv`.
- Semantic summary: `/tmp/hphys0270_full_20260603T201051Z/reports/hillslope_semantic_summary.md`.
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

Static:

- Full-suite semantic metrics are unchanged from HPHYS0269, consistent with an observability-only trace schema/package.
- The new HPHYS0270 evidence narrows continuation away from WB17 `Ep` or WB13 publication compensation. Post-execution Claude Code review further bisected H1 and localized the first high-value snowpack seam to sim-day 36 spurious melt-trigger/magnitude, not gradual accumulation.
