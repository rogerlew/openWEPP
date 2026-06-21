# Endpoint / RSS Evidence

Ran: recorded for R5C, even though R5C is not an endpoint-readiness package.

## Release Build

Ran:

```text
/usr/bin/time -f 'release_build\t%e\t%M' cargo build --release -p openwepp-runner --bin openwepp-cli-hill
sha256sum target/release/openwepp-cli-hill target/release/openwepp-cli-hill.json
```

Result:

```text
release_build 58.43 1116852
a4b2aa82b9756fce8bd83990c533b3b7da78285b7f63a36944feecb61b0eb3c3  target/release/openwepp-cli-hill
43083a215746527401b3d84802f16f5ff9b3686c468d0132c757e23821328bd4  target/release/openwepp-cli-hill.json
```

## H2637 Default-Disabled Timing

Ran:

| Rep | Seconds | RSS KB |
|---:|---:|---:|
| 1 | 639.05 | 228348 |
| 2 | 646.33 | 228100 |
| 3 | 643.96 | 228840 |

Median: `643.96 s`.

## Protected Output Comparison

Static: requested output directories under `/tmp/r5c-h2637/default/rep*/` contain
manifests only; protected outputs were written by the runfile to
`/tmp/perfmig01-final/current/anchor/h2637_same`.

Ran against `/tmp/perfdeep07/default/rep1/h2637_same`:

- `H2637.hbp`: byte-identical.
- `H2637.wat.parquet`: byte-identical.
- `H2637.pass.parquet`: bytes differ; DuckDB row equivalence passed:
  `baseline_rows=12419`, `candidate_rows=12419`, `left_minus_right=0`,
  `right_minus_left=0`, `column_count=17`.
- `H2637.loss.json`: differs only by `run_name`; normalized
  `jq -S 'del(.run_name)'` diff was empty.
- `H2637.plot.parquet`: ASCII placeholder, differs only by `run_name`;
  normalized `sed '/^run_name=/d'` diff was empty.
