# PERFIDX06 wall-clock ladder

Evidence: Ran.

## Method

OpenWEPP endpoint:

```text
/usr/bin/time -f "<case>\t%e\t%M" \
  /tmp/perfidx04/current/bin/openwepp-cli-hill \
  --run-dir <case-run-dir> \
  --run-file /tmp/perfidx06/runfiles/<case>_current.run \
  --output-dir /tmp/perfidx06/current/<case>_manifest \
  --policy compat \
  --legacy-sidecar-discovery
```

Each case removed its PERFIDX06 manifest and anchor output directory before running. Raw
times are in `/tmp/perfidx06/artifacts/wallclock-times.tsv`.

## Results

| Case | Seconds | Max RSS KB | RC |
| --- | ---: | ---: | ---: |
| `ofe1_same` | 5.33 | 21440 | 0 |
| `ofe2_same` | 9.58 | 22464 | 0 |
| `ofe3_same` | 13.76 | 23812 | 0 |
| `ofe4_same` | 22.35 | 25840 | 0 |
| `ofe5_same` | 22.21 | 25832 | 0 |
| `h2637_same` | 666.82 | 228508 | 0 |
| `h2637_with_ui_same` | 667.44 | 228640 | 0 |

## Variance Against PERFIDX04 Endpoint Sample

PERFIDX04 final endpoint timings were used as the nearest same-endpoint prior sample:

| Case | PERFIDX04 s | PERFIDX06 s | Delta s | Delta % |
| --- | ---: | ---: | ---: | ---: |
| `ofe1_same` | 5.50 | 5.33 | -0.17 | -3.09% |
| `ofe2_same` | 9.72 | 9.58 | -0.14 | -1.44% |
| `ofe3_same` | 13.92 | 13.76 | -0.16 | -1.15% |
| `ofe4_same` | 22.07 | 22.35 | 0.28 | 1.27% |
| `ofe5_same` | 22.85 | 22.21 | -0.64 | -2.80% |
| `h2637_same` | 673.29 | 666.82 | -6.47 | -0.96% |
| `h2637_with_ui_same` | 669.75 | 667.44 | -2.31 | -0.34% |

This is normal run-to-run spread for the same endpoint. PERFIDX06 does not claim a new
optimization; it re-measures the PERFIDX04 endpoint for ratio/disposition.

## Determinism Note

Raw output comparison against the PERFIDX04 anchor is recorded in
`/tmp/perfidx06/artifacts/output-hash-compare.tsv` and
`/tmp/perfidx06/artifacts/output-semantic-compare.tsv`.

Observed:

- `.hbp` and `.wat.parquet` outputs are byte-identical for all cases.
- `loss.json` payloads are equal after ignoring only the package-specific `run_name`.
- `pass.parquet` and `wat.parquet` tables are Arrow-equal with metadata ignored.
- `plot.parquet` files are ASCII text despite the extension and byte-differ by
  package-specific text content, so they are treated as opaque output-format artifacts here.

No production code changed in PERFIDX06.
