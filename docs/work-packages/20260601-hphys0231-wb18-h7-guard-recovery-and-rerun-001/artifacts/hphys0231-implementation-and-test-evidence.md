# HPHYS0231 Implementation and Test Evidence

Status: completed  
Evidence mode: Ran

## Runtime Lane

- Run root: `/tmp/hphys0231_20260601T193448Z/parity/`
- Inputs: `unpalatable-rind` runfiles (`p1..p39`)

## Cohort Execution

Ran:
1. `openwepp-cli-hill` across `H1..H39`
2. semantic comparator over candidate parquet vs baseline partitions with
   `--candidate-year-offset 2012` (using `/workdir/wepppy/.venv/bin/python`)

Observed:
- hillslope execution: `39/39` success.
- semantic reports: `39/39` generated (`rc=0`).
- overlap check: `count=39 min=1461 max=1461`.

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
- H7 guard recovery is closed.
- H1 WB18 overdrainage transient remains materially open (stream-level HOLD
  reason persists, outside HPHYS0231 closure scope).

## H7 Semantic Evidence

- `H7` candidate WAT is now produced.
- `H7.semantic.json` exists with `common_row_count=1461`.
- semantic status row for `H7` is `rc=0`.

## Summary Artifacts

Generated:
- `/tmp/hphys0231_20260601T193448Z/parity/reports/hillslope_batch_status.tsv`
- `/tmp/hphys0231_20260601T193448Z/parity/reports/semantic_status.tsv`
- `/tmp/hphys0231_20260601T193448Z/parity/reports/hillslope_semantic_summary.json`
- `/tmp/hphys0231_20260601T193448Z/parity/reports/hillslope_semantic_summary.tsv`

## Measure Mapping

- `MEASURE-HP231-005`: satisfied.
