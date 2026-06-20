# Default-Disabled Regression Gate

Status: complete.

Evidence class: Ran.

Gate: three H2637 default-disabled reps must have median `<= 676.67 s` with
direct-runtime and diagnostic environment variables unset.

Planned command shape:

```text
env -u OPENWEPP_PERFDEEP02_FRAME_ISLAND \
  -u OPENWEPP_PERFDEEP03_LANE_DENSE_STATE \
  -u OPENWEPP_PERFDEEP02_FRAME_ROUNDTRIP_PATH \
  -u OPENWEPP_INDEXED_SHADOW_REPORT_PATH \
  -u OPENWEPP_SYMBOL_REGISTRY_AUDIT_PATH \
  -u OPENWEPP_HPHYS0245_TRACE_PATH \
  /usr/bin/time -f 'r4mo_h2637_default_repN\t%e\t%M' \
  target/release/openwepp-cli-hill \
  --run-dir /tmp/perfho01/run-dirs/h2637 \
  --run-file /tmp/perfmig01-final/runfiles/h2637_same_current.run \
  --output-dir /tmp/r4mo-h2637/default/repN/h2637_same \
  --policy compat \
  --legacy-sidecar-discovery
```

## Runs

Each run used the release `openwepp-cli-hill` binary with these variables
unset:

- `OPENWEPP_PERFDEEP02_FRAME_ISLAND`
- `OPENWEPP_PERFDEEP03_LANE_DENSE_STATE`
- `OPENWEPP_PERFDEEP02_FRAME_ROUNDTRIP_PATH`
- `OPENWEPP_INDEXED_SHADOW_REPORT_PATH`
- `OPENWEPP_SYMBOL_REGISTRY_AUDIT_PATH`
- `OPENWEPP_HPHYS0245_TRACE_PATH`

```text
r4mo_h2637_default_rep1 643.70 227860
r4mo_h2637_default_rep2 646.33 229200
r4mo_h2637_default_rep3 639.62 228776
```

Median: `643.70 s`.

Gate: PASS. Median is below the `676.67 s` threshold.

Known warning retained from earlier runs: `MOFE01-MG-W-001` sidecar warning.
It does not change PASS row identity or the default-disabled regression result.
