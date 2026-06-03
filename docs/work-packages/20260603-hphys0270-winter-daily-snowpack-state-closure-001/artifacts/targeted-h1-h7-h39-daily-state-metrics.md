# Targeted H1/H7/H39 Daily State Metrics

Status: completed/HOLD
Evidence mode: ran

Ran:

- Command: `.venv/bin/python docs/work-packages/20260603-hphys0270-winter-daily-snowpack-state-closure-001/artifacts/hphys0270_diagnostics.py --run-root /tmp/hphys0270_full_20260603T201051Z --trace-max-days 180`
- Run root: `/tmp/hphys0270_full_20260603T201051Z`.
- Targeted trace status: `/tmp/hphys0270_full_20260603T201051Z/reports/targeted_trace_status.tsv`.
- Classification report: `/tmp/hphys0270_full_20260603T201051Z/reports/hphys0270_snowpack_lineage_classification.md`.

| Hill | Classification | Julian | Cand Ep | Base Ep | RM Diff | Snow-Water Diff | Pre SWE | Runtime SWE | SWE Delta | Pre Depth | Runtime Depth | Density Delta | SWE Closure Error |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| H1 | `SNOWPACK_SEMANTIC_DIVERGENCE_WITH_TRACE_CLOSED` | 99 | 0.862388 | 1.890000 | -0.003750 | -141.571250 | 0.002275 | 0.002769 | 0.000494 | 0.021096 | 0.023011 | 12.481420 | 0.000000 |
| H7 | `SNOWPACK_SEMANTIC_DIVERGENCE_WITH_TRACE_CLOSED` | 99 | 0.879520 | 1.890000 | -0.003750 | -156.821250 | 0.002275 | 0.002769 | 0.000494 | 0.021096 | 0.023011 | 12.481420 | 0.000000 |
| H39 | `SNOWPACK_SEMANTIC_DIVERGENCE_WITH_TRACE_CLOSED` | 115 | 1.790149 | 2.930000 | -13.716002 | -140.267812 | 0.003296 | 0.000962 | -0.002334 | 0.000000 | 0.000000 | 0.000000 | 0.000000 |

Static:

- H1/H7 first-material day 99 now exposes that candidate begins with only `2.275 mm` SWE and ends at `2.769 mm`, while baseline WAT `Snow-Water` is `144.34 mm` and `159.59 mm` respectively. The residual is inherited before the target day, not created by same-day closure arithmetic.
- H39 first-material day 115 begins with `3.296 mm` candidate SWE and ends with `0.962 mm`, while baseline WAT `Snow-Water` is `141.23 mm`; same-day melt is closed, but the accumulated seasonal pack is already missing.
