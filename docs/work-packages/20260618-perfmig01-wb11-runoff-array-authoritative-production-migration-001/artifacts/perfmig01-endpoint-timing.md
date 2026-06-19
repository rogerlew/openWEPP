# PERFMIG01 Endpoint Timing

Evidence: Ran + Static.

## Endpoint

Build:

```text
cargo build --release -p openwepp-runner --bin openwepp-cli-hill
```

Measured binary:

| Item | Value |
| --- | --- |
| Repo HEAD before PERFMIG01 edits | `8ee9d5d0dcd5dd98c7ec2c199d622279a93667dc` |
| PERFMIG01 measured binary | `target/release/openwepp-cli-hill` |
| PERFMIG01 binary SHA256 | `711a439a6df782cfaaeb9ca987a49cbaad098d0585ef327c2105bfb104f6b579` |
| PERFIDX06 measured binary SHA256 | `82c6cac78ed6b138b1b05750012082c1f8045602cf34004862adc48407d53e3c` |

Run:

```text
/usr/bin/time -f "h2637_same\t%e\t%M" \
  target/release/openwepp-cli-hill \
  --run-dir /tmp/perfho01/run-dirs/h2637 \
  --run-file /tmp/perfmig01-final/runfiles/h2637_same_current.run \
  --output-dir /tmp/perfmig01-final/current/h2637_same_manifest \
  --policy compat \
  --legacy-sidecar-discovery
```

Result:

| Case | Seconds | Max RSS KB | RC |
| --- | ---: | ---: | ---: |
| `h2637_same` | 669.97 | 228144 | 0 |

PERFIDX06 comparison:

| Metric | PERFIDX06 | PERFMIG01 | Delta | Delta % |
| --- | ---: | ---: | ---: | ---: |
| Seconds | 666.82 | 669.97 | +3.15 | +0.47% |
| Max RSS KB | 228508 | 228144 | -364 | -0.16% |
| Legacy ratio, no-UI median `9.12s` | 73.12x | 73.46x | +0.35x | +0.47% |

Interpretation: the first single-branch production rung is identity-clean but
does not yet improve the H2637 endpoint. This is within the package's expected
single-rung boundary-offset class, not a `REDIRECT` condition.

## Transitional Boundary Measurement

The transition boundary was measured with an artifact-local harness that calls
the production `evaluate_indexed_kernel_writeback` and
`apply_indexed_kernel_writeback` functions on a 543 state + 8 flux id-backed
payload.

Ran:

```text
cargo run --release \
  --manifest-path docs/work-packages/20260618-perfmig01-wb11-runoff-array-authoritative-production-migration-001/artifacts/perfmig01-transition-boundary-bench/Cargo.toml \
  -- 50000
```

Saved raw result: `perfmig01-transition-boundary-bench.tsv`.

| Metric | Repetitions | us/payload | ns/field | Projected H2637 seconds |
| --- | ---: | ---: | ---: | ---: |
| `evaluate_indexed_payload` | 50000 | 3.297865 | 5.985236 | 0.778168 |
| `apply_indexed_payload` | 50000 | 107.531649 | 195.157257 | 25.373275 |
| `evaluate_plus_apply_indexed_payload` | 50000 | 111.894193 | 203.074760 | 26.402666 |

The apply path is the compatibility materialization boundary: id value write,
registry symbol resolution, and updated-symbol logical map insert. The projection
uses the full H2637 `235961` OFE-day count as a conservative accounting scale;
actual activation is branch-dependent because active snow/frost/irrigation/MOFE
carry days are still logical in this rung.

## Trajectory

PERFMIG01 should not be read as full endpoint closure. The measured transition
boundary is large enough to hide or exceed a single WB11 branch win while only
one production phase is migrated. The next rung should migrate adjacent WB11
consumers as a contiguous cluster so the updated WB11 outputs stay dense across
more of the daily hydrology chain and this compatibility boundary moves outward.
