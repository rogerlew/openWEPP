# PERFMIG02 Endpoint Timing

Static: timing compared against PERFMIG01 and PERFIDX06 package anchors.

Ran: release build, H2637 no-UI endpoint run, and artifact-local transition-boundary bench.

## H2637 Endpoint

Ran:

```text
cargo build --release -p openwepp-runner --bin openwepp-cli-hill
sha256sum target/release/openwepp-cli-hill
/usr/bin/time -f "h2637_same\t%e\t%M" \
  target/release/openwepp-cli-hill \
  --run-dir /tmp/perfho01/run-dirs/h2637 \
  --run-file /tmp/perfmig01-final/runfiles/h2637_same_current.run \
  --output-dir /tmp/perfmig02-final/current/h2637_same_manifest \
  --policy compat \
  --legacy-sidecar-discovery
```

Result:

```text
d4f7603e79fdf415e3e4123a2baa7df19a6cb7780e8d01206bfaad6ef012d63b  target/release/openwepp-cli-hill
h2637_same          672.14    227636
h2637_same_repeat   675.00    228152
```

Comparison:

| Anchor | Seconds | RSS KB | Ratio vs 9.12 s legacy no-UI | Delta vs PERFMIG02 |
|---|---:|---:|---:|---:|
| PERFIDX06 | 666.82 | 228508 | 73.12x | PERFMIG02 is +5.32 to +8.18 s (+0.80% to +1.23%) |
| PERFMIG01 | 669.97 | 228144 | 73.46x | PERFMIG02 is +2.17 to +5.03 s (+0.32% to +0.75%) |
| PERFMIG02 final run 1 | 672.14 | 227636 | 73.70x | current final binary |
| PERFMIG02 final run 2 | 675.00 | 228152 | 74.01x | repeat final binary |

Endpoint verdict: FAIL/REDIRECT. Two final-code H2637 runs were negative versus PERFMIG01 and PERFIDX06.

## Transition-Boundary Bench

Ran:

```text
cargo run --release \
  --manifest-path docs/work-packages/20260618-perfmig02-wb11-consumer-cluster-boundary-retirement-001/artifacts/perfmig02-transition-boundary-bench/Cargo.toml \
  -- 50000
```

Result file: `perfmig02-transition-boundary-bench.tsv`.

```text
metric                                             repetitions  elapsed_s    us_per_payload  ns_per_field  projected_h2637_s
evaluate_indexed_payload                          50000        0.172667414  3.453348        6.267420      0.814856
apply_indexed_payload_materialize_all             50000        5.237616801  104.752336      190.113133    24.717466
apply_indexed_payload_perfmig02_skip_6            50000        5.273025488  105.460510      191.398384    24.884567
evaluate_plus_apply_indexed_payload_perfmig02_skip_6 50000     5.577987310  111.559746      202.467779    26.323749
```

Boundary attribution verdict: FAIL for the package's strict "apply cost drops" subgate. The conservative
six-symbol materialization retirement is `+0.708174 us/payload` versus materialize-all in this bench, about
`+0.167101 s` projected over H2637 OFE-days. The reason is intentional: the fail-closed stale-logical removal
work is more expensive than avoiding six logical inserts.

## Interpretation

The endpoint did not improve on the final clippy-clean code. A pre-clippy intermediate binary measured
`656.09 s`, but that binary is superseded and is not used for disposition. The final binary measured
`672.14 s` and `675.00 s`, so PERFMIG02 triggers the package's REDIRECT condition.

This means the widen-and-retire path, as authored here, did not convert fast enough. The next perf package
should pivot to a deeper array-native phase path where dense read + compute + write are captured together.
