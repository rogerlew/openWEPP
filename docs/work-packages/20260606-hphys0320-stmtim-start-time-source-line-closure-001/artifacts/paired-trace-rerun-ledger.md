# Paired Trace Rerun Ledger

Status: complete

Evidence mode: Ran

Ran:

Release binary:

```sh
cargo build --release -p openwepp-runner --bin openwepp-cli-hill
```

Result: passed.

Focused trace rerun:

```sh
OPENWEPP_HPHYS0245_TRACE_PATH=<trace> \
OPENWEPP_HPHYS0245_TRACE_MAX_DAYS=1800 \
target/release/openwepp-cli-hill \
  --run-dir /tmp/hphys0305_paired_melt_terms_20260605T000000Z/runs \
  --run-file p<1|7|39>_openwepp.run \
  --output-dir /tmp/hphys0320_stmtim_start_time_source_line_20260606T000000Z/hillslope_output \
  --policy compat
```

Trace paths:

- `/tmp/hphys0320_stmtim_start_time_source_line_20260606T000000Z/hillslope_output/H1.hphys0320.trace.jsonl`
- `/tmp/hphys0320_stmtim_start_time_source_line_20260606T000000Z/hillslope_output/H7.hphys0320.trace.jsonl`
- `/tmp/hphys0320_stmtim_start_time_source_line_20260606T000000Z/hillslope_output/H39.hphys0320.trace.jsonl`

HPHYS0319 focus row, 2013 day 11 hour 11:

| Hillslope | rain m | stmdur s | wntdur | wnttim | active | snow branch | hrrain m | hrsnow m |
|---|---:|---:|---:|---:|---:|---:|---:|---:|
| H1 | `0.00082` | `38040.12` | `11` | `1` | `1` | `1` | `0` | `0.0007454545454545453` |
| H7 | `0.00082` | `38040.12` | `11` | `1` | `1` | `1` | `0` | `0.0007454545454545453` |
| H39 | `0.00082` | `38040.12` | `11` | `1` | `1` | `1` | `0` | `0.0007454545454545453` |

Closure tokens:

- `wntdur = 11`
- `wnttim = 1`
- active interval `1`
- snow branch `1`
- hrsnow ~= `0.00074545 m`
- carried_rows_closed_for_timing_seam: `57`

The remaining deltas versus the HPHYS0319 fixed-baseline observe lane are
non-timing scalar differences already outside this seam: OpenWEPP reports
`stmdur_s = 38040.12` versus fixed-baseline `38040.0`, and hourly temperature
differs only at the already observed sub-millidegree scale.
