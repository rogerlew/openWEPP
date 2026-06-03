# Targeted H1/H7/H39 Diagnostics

Status: completed

Evidence mode: ran

## Command

Ran:

```text
/workdir/wepppy/.venv/bin/python docs/work-packages/20260602-hphys0254-wb11-initial-storage-projection-closure-001/artifacts/hphys0254_diagnostics.py --run-root /tmp/hphys0257_20260603T020345Z
```

## Outputs

- Ran: H1 trace
  `/tmp/hphys0257_20260603T020345Z/hillslope_output/H1.hphys0254.trace.jsonl`.
- Ran: H7 trace
  `/tmp/hphys0257_20260603T020345Z/hillslope_output/H7.hphys0254.trace.jsonl`.
- Ran: H39 trace
  `/tmp/hphys0257_20260603T020345Z/hillslope_output/H39.hphys0254.trace.jsonl`.
- Ran: targeted report
  `/tmp/hphys0257_20260603T020345Z/reports/targeted_h1_h7_h39_storage_summary.md`.

## Day-1 Targeted Residuals

| Hillslope | post_seed - baseline t0 mm | Total-Soil diff mm | Dp diff mm | latqcc diff mm | Ep diff mm |
| --- | ---: | ---: | ---: | ---: | ---: |
| H1 | 0.015748 | -0.247876 | 0.004798 | 0.023532 | 0.235294 |
| H7 | 0.078917 | -0.209171 | 0.004800 | 0.047995 | 0.235294 |
| H39 | 0.084258 | -0.336200 | 0.004800 | 0.180364 | 0.235294 |

## Interpretation

- Ran: targeted H1/H7/H39 day-1 `latqcc` and `Total-Soil` residuals improved
  materially versus HPHYS0256.
- Static: residuals remain non-zero; treat the next surface as hourly
  cap/withdrawal/publication lineage until instrumentation proves otherwise.
