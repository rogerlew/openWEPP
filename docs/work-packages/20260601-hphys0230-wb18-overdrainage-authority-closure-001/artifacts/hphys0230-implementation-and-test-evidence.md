# HPHYS0230 Implementation and Test Evidence

Status: completed  
Evidence mode: Ran

## Runtime Lane

- Run root: `/tmp/hphys0230_20260601T183925Z/parity/`
- Inputs: `unpalatable-rind` runfiles (`p1..p39`)

## Cohort Execution

Ran:
1. `openwepp-cli-hill` across `H1..H39`
2. semantic comparator over candidate parquet vs baseline partitions with
   `--candidate-year-offset 2012` (using `/workdir/wepppy/.venv/bin/python`)

Observed:
- hillslope execution: `38/39` success.
- failing hillslope: `H7` (`HKERNEL-WB11-PERC-E-003` in WB18 percolation).
- semantic reports: `38` generated; `H7` report missing because
  `H7.wat.parquet` was not produced.
- overlap check (generated reports only):
  `count=38 min=1461 max=1461`.

## H1 Acceptance Trace (WB18 transient)

Compared `H1` early days (candidate vs baseline):

| day | `Dp` candidate | `Dp` baseline | `Total-Soil` candidate | `Total-Soil` baseline |
| ---: | ---: | ---: | ---: | ---: |
| 1 | 39.525 | 0.240 | 254.021 | 343.070 |
| 2 | 67.128 | 0.240 | 186.632 | 342.470 |
| 3 | 41.265 | 0.240 | 145.254 | 341.720 |
| 4 | 13.805 | 0.240 | 131.926 | 340.920 |
| 5 | 20.195 | 0.240 | 111.165 | 340.090 |
| 6 | 11.799 | 0.240 | 99.974 | 339.210 |
| 7 | 7.766 | 0.240 | 92.273 | 338.220 |

Interpretation:
- dynamic `Bi` reduced the day-2 spike from prior `82.52` to `67.13`,
  but did not remove the overdrainage regime.

## Summary Artifact

Generated:
- `/tmp/hphys0230_20260601T183925Z/parity/reports/hillslope_semantic_summary.json`

Notes:
- summary denominator is `38` reports due `H7` runtime failure.

## Measure Mapping

- `MEASURE-HP230-004`: **not satisfied**.
- `MEASURE-HP230-005`: **not satisfied** (`H7` missing comparator artifact).
