# PERFMIG01 Bit Identity

Evidence: Ran + Static.

## Focused Identity Fixture

Ran:

```text
cargo test -p openwepp-hillslope-orchestrator perfmig01_wb11_warm_rain_indexed_writeback_is_bit_identical -- --nocapture
```

Result: pass.

The fixture builds the PERFARCH03 warm-rain WB11 runoff surface, runs the
production logical path, then reruns the same request with indexed state/flux
authority and hot-symbol tables.

Observed assertions:

| Check | Result |
| --- | --- |
| Logical response status | OK |
| Logical response indexed payload | `None` |
| Logical state update count | 543 |
| Logical flux update count | 8 |
| Indexed response status | OK |
| Indexed logical payload counts | `0 state`, `0 flux` |
| Indexed state update count | 543 |
| Indexed flux update count | 8 |
| Materialized state map equality | exact |
| Materialized flux map equality | exact |
| Per-value `f64::to_bits()` equality | exact |

## Scheduler Fixture

Ran:

```text
cargo test -p openwepp-hillslope-orchestrator perfmig01_scheduler_applies_indexed_writeback_payload -- --nocapture
```

Result: pass.

This test uses a custom kernel that returns `KernelRunResponse::with_indexed_writeback`
on every scheduler phase. The scheduler sees indexed state/flux request surfaces,
applies the id-backed payload, and materializes the updated state and flux symbols
to the logical compatibility maps.

## H2637 Output Identity

Ran:

```text
/usr/bin/time -f "h2637_same\t%e\t%M" \
  target/release/openwepp-cli-hill \
  --run-dir /tmp/perfho01/run-dirs/h2637 \
  --run-file /tmp/perfmig01-final/runfiles/h2637_same_current.run \
  --output-dir /tmp/perfmig01-final/current/h2637_same_manifest \
  --policy compat \
  --legacy-sidecar-discovery
```

Observed output comparison against the PERFIDX06 anchor:

| Output | Comparison |
| --- | --- |
| `H2637.hbp` | byte-identical |
| `H2637.wat.parquet` | byte-identical |
| `H2637.pass.parquet` | Arrow table equal with metadata ignored |
| `H2637.loss.json` | differs only by isolated runfile `run_name` |
| `H2637.plot.parquet` | ASCII sidecar differs only by isolated runfile `run_name` |

Ran with `pyarrow`:

```text
pass rows 12419 12419 cols 17 17
pass schema_equal True table_equal True
wat rows 235961 235961 cols 34 34
wat schema_equal True table_equal True
```

## Branch Boundary

The only migrated production branch is the warm-rain branch. Active snow,
frost, irrigation, and MOFE hourly carry branches remain on the logical payload
path and are explicitly named in `perfmig01-migration.md`.
