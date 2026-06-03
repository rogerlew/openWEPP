# Full 39 Suite Metrics

Status: completed

Evidence mode: ran

Ran:

```text
/workdir/wepppy/.venv/bin/python docs/work-packages/20260603-hphys0259-wb19-trace-ep-dp-storage-localization-closure-001/artifacts/hphys0259_diagnostics.py --run-root /tmp/hphys0259_20260603T031427Z
```

Reports:

- Ran: `/tmp/hphys0259_20260603T031427Z/reports/hillslope_semantic_summary.md`.
- Ran: `/tmp/hphys0259_20260603T031427Z/reports/targeted_h1_h7_h39_storage_summary.md`.

## Summary

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

## Targeted Day-1 Residuals

| Hillslope | Total-Soil diff mm | Dp diff mm | latqcc diff mm | Ep diff mm |
| --- | ---: | ---: | ---: | ---: |
| H1 | -0.247876 | 0.004798 | 0.023532 | 0.235294 |
| H7 | -0.209171 | 0.004800 | 0.047995 | 0.235294 |
| H39 | -0.336200 | 0.004800 | 0.180364 | 0.235294 |

## Comparison

- Ran: metrics are unchanged from HPHYS0258 because HPHYS0259 added trace
  evidence propagation, not numerical flux compensation.
