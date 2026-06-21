# Default-Disabled Regression Gate

## Release Build

Ran:

```text
/usr/bin/time -f 'release_build %e %M' cargo build --release -p openwepp-runner --bin openwepp-cli-hill
sha256sum target/release/openwepp-cli-hill target/release/openwepp-cli-hill.json
```

Result:

```text
release_build 58.07 1117172
2223d2fd0cdfbfe1216e52704ddf5c822a6f800693e092a7488535e8dbd3d1bc  target/release/openwepp-cli-hill
6bc145bc3ae4289360c9d4ceee6b9efed397c62e09bfb7ce9ca93088bbdd65bf  target/release/openwepp-cli-hill.json
```

## H2637 Default-Disabled Runtime

All direct-runtime and diagnostic env vars were unset. Command shape:

```text
env \
  -u OPENWEPP_PERFDEEP02_FRAME_ISLAND \
  -u OPENWEPP_PERFDEEP03_LANE_DENSE_STATE \
  -u OPENWEPP_PERFDEEP02_FRAME_ROUNDTRIP_PATH \
  -u OPENWEPP_INDEXED_SHADOW_REPORT_PATH \
  -u OPENWEPP_SYMBOL_REGISTRY_AUDIT_PATH \
  -u OPENWEPP_HPHYS0245_TRACE_PATH \
  /usr/bin/time -f 'r5d_h2637_default_repN\t%e\t%M' \
  target/release/openwepp-cli-hill \
  --run-dir /tmp/perfho01/run-dirs/h2637 \
  --run-file /tmp/perfmig01-final/runfiles/h2637_same_current.run \
  --output-dir /tmp/r5d-h2637/default/repN/h2637_same \
  --policy compat \
  --legacy-sidecar-discovery
```

Results:

| Rep | Seconds | RSS KB | Warning |
|---|---:|---:|---|
| 1 | 647.54 | 228652 | known `MOFE01-MG-W-001` |
| 2 | 647.93 | 228384 | known `MOFE01-MG-W-001` |
| 3 | 644.88 | 229340 | known `MOFE01-MG-W-001` |

Median: `647.54 s`.

Threshold: `<= 676.67 s`.

Verdict: PASS.

## Protected Output Comparison

Candidate: `/tmp/perfmig01-final/current/anchor/h2637_same`.

Baseline: `/tmp/perfdeep07/default/rep1/h2637_same`.

- `H2637.hbp`: byte-identical.
- `H2637.wat.parquet`: byte-identical.
- `H2637.pass.parquet`: byte differs; DuckDB row equivalence PASS:
  - baseline rows: `12419`
  - candidate rows: `12419`
  - left-minus-right: `0`
  - right-minus-left: `0`
  - column count: `17`
- `H2637.loss.json`: normalized `jq -S 'del(.run_name)'` diff empty.
- `H2637.plot.parquet`: normalized `sed '/^run_name=/d'` diff empty.
