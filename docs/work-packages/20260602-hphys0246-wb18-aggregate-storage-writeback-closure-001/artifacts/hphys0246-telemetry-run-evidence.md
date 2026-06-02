# HPHYS0246 Telemetry Run Evidence

Status: completed
Evidence mode: Ran

## Run Root
- `/tmp/hphys0246_20260602T053935Z`

## Inputs
- Source run directory: `/tmp/hphys0245_20260602T051933Z/runs`
- Copied run files into `/tmp/hphys0246_20260602T053935Z/runs`
- Reused the HPHYS0245 diagnostics sidecar:
  - `OPENWEPP_HPHYS0245_TRACE_PATH`
  - `OPENWEPP_HPHYS0245_TRACE_MAX_DAYS=30`

## Command Shape
- `target/debug/openwepp-cli-hill --run-dir /tmp/hphys0246_20260602T053935Z/runs --run-file p1_openwepp.run --output-dir /tmp/hphys0246_20260602T053935Z/hillslope_output --policy compat`
- Same command shape for `p7_openwepp.run` and `p39_openwepp.run`.

## Results
| Hillslope | Return Code | Trace Rows |
| --- | --- | --- |
| H1 | 0 | 480 |
| H7 | 0 | 480 |
| H39 | 0 | 480 |

## Generated Reports
- `/tmp/hphys0246_20260602T053935Z/reports/telemetry_status.tsv`
- `/tmp/hphys0246_20260602T053935Z/reports/hphys0246_trace_all_rows.tsv`
- `/tmp/hphys0246_20260602T053935Z/reports/hphys0246_storage_balance_first30.tsv`
- `/tmp/hphys0246_20260602T053935Z/reports/hphys0246_storage_balance_summary.tsv`
- `/tmp/hphys0246_20260602T053935Z/reports/hphys0246_before_after_summary.tsv`
- `/tmp/hphys0246_20260602T053935Z/reports/hphys0246_phase_storage_delta_summary.tsv`
