# Full 39 Suite Metrics

Status: completed/HOLD
Evidence mode: ran

Static:

- HPHYS0272 changes the radiation-unit seam only; broader semantic residuals
  are continuation evidence, not rejection of the scoped fix.

Ran:

- Run root: `/tmp/hphys0272_full_20260603T221209Z`
- Runtime status:
  `/tmp/hphys0272_full_20260603T221209Z/reports/hillslope_batch_status.tsv`
- Semantic status:
  `/tmp/hphys0272_full_20260603T221209Z/reports/semantic_status.tsv`
- Semantic summary:
  `/tmp/hphys0272_full_20260603T221209Z/reports/hillslope_semantic_summary.md`

## Runtime

- Build `cargo build -p openwepp-runner --bin openwepp-cli-hill`: `rc=0`,
  `0.192s`.
- Targeted H1/H7/H39 traces: `3/3 rc=0`.
- H1..H39 runtime batch: `39/39 rc=0`.
- Semantic comparator command status: `39/39 rc=0`.
- Semantic parity pass: `0/39`.

## Selected Semantic Residuals

| Symbol | Pass Hillslopes | Total Fail Count | Mean Abs Diff Mean | Max Abs Diff |
| --- | --- | --- | --- | --- |
| Ep | 0/39 | 56132 | 1.669264 | 7.778863 |
| Total-Soil | 0/39 | 55908 | 149.442866 | 611.813445 |
| SoilWaterTotal | 0/39 | 55908 | 149.442866 | 611.813445 |
| Dp | 0/39 | 35445 | 0.150040 | 0.244800 |
| latqcc | 0/39 | 40340 | 0.675265 | 14.760000 |
| Q | 0/39 | 5547 | 1.245240 | 43.926329 |
| RM | 0/39 | 7349 | 0.324492 | 41.480927 |
| Snow-Water | 0/39 | 13799 | 4.909469 | 102.625114 |

## Delta From HPHYS0271

| Symbol | HPHYS0271 Max Abs Diff | HPHYS0272 Max Abs Diff | Interpretation |
| --- | --- | --- | --- |
| RM | `203.969200` | `41.480927` | large reduction from removing false radiation-driven melt release |
| Snow-Water | `560.770686` | `102.625114` | large reduction from preserving snowpack instead of releasing spurious melt |
| Q | `193.834417` | `43.926329` | runoff residual reduced with melt timing correction |
| Ep | `7.778863` | `7.778863` | unchanged; remains continuation focus |
| Total-Soil | `611.813445` | `611.813445` | unchanged; remains continuation focus |
