# Full 39 Suite Metrics

Status: completed

Evidence mode: ran

## Command

Ran:

```text
/workdir/wepppy/.venv/bin/python docs/work-packages/20260602-hphys0254-wb11-initial-storage-projection-closure-001/artifacts/hphys0254_diagnostics.py --run-root /tmp/hphys0258_20260603T023606Z
```

## Summary

- Ran: semantic summary
  `/tmp/hphys0258_20260603T023606Z/reports/hillslope_semantic_summary.md`.
- Ran: semantic pass `0/39`.

| Symbol | Pass Hillslopes | Total Fail Count | Mean Abs Diff Mean | Max Abs Diff |
| --- | ---: | ---: | ---: | ---: |
| Ep | 0/39 | 56416 | 1.689334 | 7.779383 |
| Total-Soil | 0/39 | 56402 | 152.388768 | 616.171444 |
| SoilWaterTotal | 0/39 | 56402 | 152.388768 | 616.171444 |
| Dp | 0/39 | 35698 | 0.151072 | 0.244800 |
| latqcc | 0/39 | 40227 | 0.675393 | 14.760000 |
| Q | 0/39 | 2986 | 0.925027 | 194.715728 |
| RM | 0/39 | 10678 | 2.301802 | 204.850510 |
| Snow-Water | 0/39 | 24137 | 58.195696 | 562.470000 |

## Comparison

- Ran: metrics are unchanged from HPHYS0257
  `/tmp/hphys0257_20260603T020345Z`.
- Static: unchanged metrics are expected because HPHYS0258 did not alter
  baseline-authoritative WB19 equations or downstream WB13 numerical
  publication.
