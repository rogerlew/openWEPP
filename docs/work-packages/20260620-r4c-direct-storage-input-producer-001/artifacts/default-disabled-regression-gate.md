# R4C Default-Disabled Regression Gate

Status: complete.
Evidence mode: Ran.

Gate:

- H2637 default-disabled median must be `<= 676.67 s`.
- Protected output identity must pass.
- PASS parquet row equivalence must pass when parquet bytes vary.

Required command shape:

```text
target/release/openwepp-cli-hill \
  --run-dir /tmp/perfho01/run-dirs/h2637 \
  --run-file /tmp/perfmig01-final/runfiles/h2637_same_current.run \
  --output-dir <r4c-output-root>/default/repN/h2637_same \
  --policy compat \
  --legacy-sidecar-discovery
```

Required disabled env posture:

- unset `OPENWEPP_PERFDEEP02_FRAME_ISLAND`
- unset `OPENWEPP_PERFDEEP03_LANE_DENSE_STATE`
- unset `OPENWEPP_PERFDEEP02_FRAME_ROUNDTRIP_PATH`
- unset `OPENWEPP_INDEXED_SHADOW_REPORT_PATH`
- unset `OPENWEPP_SYMBOL_REGISTRY_AUDIT_PATH`
- unset `OPENWEPP_HPHYS0245_TRACE_PATH`

## Release Build

Ran:

```text
/usr/bin/time -f 'release_build\t%e\t%M' cargo build --release -p openwepp-runner --bin openwepp-cli-hill
```

Result:

```text
release_build 57.28 1103576
```

Release identity:

```text
2b9dd7077f41437902f6b8c6fb3e5789cfad816c2616e463dc60dcb9270b2d7c  target/release/openwepp-cli-hill
2b8bc0418a752d5f93636f8ee2589bb59e0ab9efcbe804648546c646219bc517  target/release/openwepp-cli-hill.json
```

Note: manifests record current `HEAD` as
`1fab60093a3058247689a77bfaf78b71c148dddb`; the benchmark binary identity above
is the authoritative identity for this uncommitted R4C execution tree.

## H2637 Default-Disabled Reps

Output root:

```text
/tmp/r4c-h2637-final/default
```

| Rep | Elapsed seconds | Peak RSS KB | Notes |
|---|---:|---:|---|
| 1 | `637.63` | `229136` | known `MOFE01-MG-W-001` warning only |
| 2 | `640.25` | `227532` | known `MOFE01-MG-W-001` warning only |
| 3 | `639.19` | `229220` | known `MOFE01-MG-W-001` warning only |

Median: `639.19 s`.

Threshold: `<= 676.67 s`.

Verdict: PASS.

## Protected Identity

Final current output hashes:

```text
44acc83b025b7a7ed9df3ad77f2d595a17f7e59ae923a1224f8ee294ad09bfe8  H2637.hbp
4d4421a2dcc1275af607059605249517d7f605f4431644aa4e675966daf8e021  H2637.loss.json
d66e811626b10078a79c35fd1a60e499ca1d09a46eca76395b2b67157385127b  H2637.pass.parquet
1419d03fad4b5f8dbd8aad6aabae95a6c10934a9e4d7f8ef65437968a12926d6  H2637.plot.parquet
c70af52324b52c89119e57524f75bf4875d2c6a9ff83fe56d239a22082b9b474  H2637.wat.parquet
```

PASS parquet byte hashes varied across reps, so semantic identity was checked
with DuckDB against the established baseline:

| baseline_rows | candidate_rows | left_minus_right | right_minus_left |
|---:|---:|---:|---:|
| 12419 | 12419 | 0 | 0 |

Column count: `17`.

Verdict: PASS.
