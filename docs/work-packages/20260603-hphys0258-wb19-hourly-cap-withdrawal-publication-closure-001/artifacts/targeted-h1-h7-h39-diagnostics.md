# Targeted H1/H7/H39 Diagnostics

Status: completed

Evidence mode: ran

Ran:

```text
/workdir/wepppy/.venv/bin/python docs/work-packages/20260602-hphys0254-wb11-initial-storage-projection-closure-001/artifacts/hphys0254_diagnostics.py --run-root /tmp/hphys0258_20260603T023606Z
```

Targeted report:

- Ran: `/tmp/hphys0258_20260603T023606Z/reports/targeted_h1_h7_h39_storage_summary.md`.

| Hillslope | Total-Soil diff mm | Dp diff mm | latqcc diff mm | Ep diff mm |
| --- | ---: | ---: | ---: | ---: |
| H1 | -0.247876 | 0.004798 | 0.023532 | 0.235294 |
| H7 | -0.209171 | 0.004800 | 0.047995 | 0.235294 |
| H39 | -0.336200 | 0.004800 | 0.180364 | 0.235294 |

- Ran: targeted metrics are unchanged from HPHYS0257 because HPHYS0258 added
  diagnostic publication, not numerical flux compensation.
