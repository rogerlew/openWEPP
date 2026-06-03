# Targeted H1/H7/H39 Snowpack Baseline Metrics

Status: completed/HOLD
Evidence mode: ran

Ran:

- Command: `.venv/bin/python docs/work-packages/20260603-hphys0269-winter-melt-snowpack-baselining-001/artifacts/hphys0269_diagnostics.py --run-root /tmp/hphys0269_full_final_20260603T185740Z --trace-max-days 180`
- Run root: `/tmp/hphys0269_full_final_20260603T185740Z`.
- Build: `cargo build -p openwepp-runner --bin openwepp-cli-hill` returned `0`.
- Targeted traces: H1, H7, and H39 returned `0`.
- Classification report: `/tmp/hphys0269_full_final_20260603T185740Z/reports/hphys0269_snowpack_lineage_classification.md`.

| Hill | Classification | Julian | Cand Ep | Base Ep | RM Diff | Snow-Water Diff | S | Melt | Raw Melt | Retained Rain | SWE Closure Error |
|---|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| H1 | `SNOWPACK_SEMANTIC_DIVERGENCE_WITH_TRACE_CLOSED` | 99 | 0.862388 | 1.890000 | -0.003750 | -141.571250 | -0.000494 | 0.000000 | 0.000000 | 0.000000 | 0.000000 |
| H7 | `SNOWPACK_SEMANTIC_DIVERGENCE_WITH_TRACE_CLOSED` | 99 | 0.879520 | 1.890000 | -0.003750 | -156.821250 | -0.000494 | 0.000000 | 0.000000 | 0.000000 | 0.000000 |
| H39 | `SNOWPACK_SEMANTIC_DIVERGENCE_WITH_TRACE_CLOSED` | 115 | 1.790149 | 2.930000 | -13.716002 | -140.267812 | 0.002334 | 0.002334 | 0.004473 | 0.000000 | 0.000000 |

Static: compared to HPHYS0268, H39 improved from `RM diff=-16.05 mm` and
`Snow-Water diff=-141.23 mm` to `RM diff=-13.716002 mm` and
`Snow-Water diff=-140.267812 mm`. H1/H7 first-material divergence metrics are
unchanged, so the migrated slice is necessary but insufficient.
