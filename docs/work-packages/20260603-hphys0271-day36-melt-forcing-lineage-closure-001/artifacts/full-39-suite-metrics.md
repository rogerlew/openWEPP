# Full 39 Suite Metrics

Status: completed/HOLD
Evidence mode: ran

Ran:

- Run root: `/tmp/hphys0271_full_20260603T212901Z`
- Runtime status: `/tmp/hphys0271_full_20260603T212901Z/reports/hillslope_batch_status.tsv`
- Semantic status: `/tmp/hphys0271_full_20260603T212901Z/reports/semantic_status.tsv`
- Semantic summary: `/tmp/hphys0271_full_20260603T212901Z/reports/hillslope_semantic_summary.md`

## Runtime

- Build `cargo build -p openwepp-runner --bin openwepp-cli-hill`: `rc=0`, `0.418s`.
- H1..H39 runtime batch: `39/39` hillslopes returned `rc=0`.
- Semantic comparator command status: `39/39` returned `rc=0`.
- Semantic parity pass: `0/39`.

## Selected Semantic Residuals

| Symbol | Pass Hillslopes | Total Fail Count | Mean Abs Diff Mean | Max Abs Diff |
| --- | --- | --- | --- | --- |
| Ep | 0/39 | 56132 | 1.669264 | 7.778863 |
| Total-Soil | 0/39 | 55908 | 149.442866 | 611.813445 |
| SoilWaterTotal | 0/39 | 55908 | 149.442866 | 611.813445 |
| Dp | 0/39 | 35445 | 0.150040 | 0.244800 |
| latqcc | 0/39 | 40340 | 0.675265 | 14.760000 |
| Q | 0/39 | 4480 | 0.979774 | 193.834417 |
| RM | 0/39 | 10367 | 2.272853 | 203.969200 |
| Snow-Water | 0/39 | 23976 | 56.627822 | 560.770686 |
