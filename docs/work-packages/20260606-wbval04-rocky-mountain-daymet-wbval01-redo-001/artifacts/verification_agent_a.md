# Verification Agent A

Status: complete

Evidence mode: mixed `Static:` and `Ran:`

Verification focus: independently verify the climate precondition audit and
run manifest against local artifacts.

Ran:

- Re-read `/wc1/runs/in/indispensable-presenter/climate/wepp_cli.parquet` and
  `/wc1/runs/in/indispensable-presenter/climate/wepp.cli`.
- Recomputed the baseline `sunmap.r3` row-by-row for latitude `43.73`.
- Verified zero text CLI and parquet CLI radiation-bound exceedances.
- Verified text CLI rows: `2191`; parquet rows: `2191`; text/parquet `rad`
  mismatch count: `0`.
- Inspected `/tmp/wbval04_rocky_mountain_20260606T000000Z/run_status.tsv`.

Verification:

| Check | Result | Evidence |
|---|---|---|
| Climate precondition reproduced | pass | zero `rad > sunmap.r3`; minimum margin `0.000293 Ly/day`. |
| Run manifest paths exist | pass | `/tmp/wbval04_rocky_mountain_20260606T000000Z/` contains wrappers, outputs, stdout/stderr, binary hash, and status TSV. |
| Commands and exit statuses are traceable | pass | `run_status.tsv` records `18` RC `0` WAT emitters and `4` RC `1` fail-closed runs. |

No verification exceptions.
