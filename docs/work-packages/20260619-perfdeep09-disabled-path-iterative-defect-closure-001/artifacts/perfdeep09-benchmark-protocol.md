# PERFDEEP09 Benchmark Protocol

Status: complete.
Evidence class: Ran.

Release build:

```text
cargo build --release -p openwepp-runner --bin openwepp-cli-hill
```

Default-disabled environment:

```text
env -u OPENWEPP_PERFDEEP02_FRAME_ISLAND \
    -u OPENWEPP_PERFDEEP03_LANE_DENSE_STATE \
    -u OPENWEPP_PERFDEEP02_FRAME_ROUNDTRIP_PATH \
    -u OPENWEPP_INDEXED_SHADOW_REPORT_PATH \
    -u OPENWEPP_SYMBOL_REGISTRY_AUDIT_PATH \
    -u OPENWEPP_HPHYS0245_TRACE_PATH
```

Common command shape:

```text
/usr/bin/time -f "<label>\t%e\t%M" \
  <default-disabled-env> \
  target/release/openwepp-cli-hill \
  --run-dir /tmp/perfho01/run-dirs/h2637 \
  --run-file /tmp/perfmig01-final/runfiles/h2637_same_current.run \
  --output-dir <manifest-output-dir> \
  --policy compat \
  --legacy-sidecar-discovery
```

Threshold:

- Single-run candidates are screening only.
- `READY-FOR-R2` requires three final default-disabled H2637 reps with median
  `<= 676.67 s`.

PASS parquet identity uses the established PERFDEEP policy: raw parquet bytes
may drift; schema and row equivalence are checked with Arrow/DuckDB. HBP, WAT,
plot, and loss are byte-checked for this same runfile.
