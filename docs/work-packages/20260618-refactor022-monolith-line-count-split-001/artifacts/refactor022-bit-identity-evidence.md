# REFACTOR022 Bit-Identity Evidence

Evidence class: Ran.

## Baseline Selection

Initial comparisons against PERFIDX05 anchors failed because those artifacts were stale for
REFACTOR022: their run files and source state did not match the current pre-refactor `HEAD`.
Example differences included `run_name`, snow override metadata, and H2637 table drift from
later clean-HEAD changes. Those failed diagnostics are retained under
`/tmp/refactor022/artifacts/current-rerun-identity.tsv` and
`/tmp/refactor022/artifacts/current-final-baseline-identity.tsv`, but they are not the
acceptance baseline.

The accepted baseline was built from the true pre-refactor tree:

- Source snapshot: `git archive HEAD` into `/tmp/refactor022/head-baseline-src`.
- Baseline build: `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`.
- Build result: pass, `1m35s`.
- Refactored build: `cargo build --release -p openwepp-runner --bin openwepp-cli-hill`,
  pass, `57.36s`.
- Inputs: the same REFACTOR022 run files and run directories, with only output roots
  rewritten between baseline and refactored runs.

## Anchor Result

Raw comparator:
`/tmp/refactor022/artifacts/head-baseline-vs-refactor022-identity.tsv`.

```text
anchor_mismatches    0
```

Acceptance interpretation:

- HBP: byte-identical for all seven cases.
- Loss JSON: byte-identical for all seven cases.
- Plot output: byte-identical for all seven cases.
- WAT parquet: byte-identical and table-equal for all seven cases.
- PASS parquet: table-equal for all seven cases; byte container churn is expected.

## Case Summary

| Case | HBP | Loss | Plot | WAT byte/table | PASS table | PASS rows | WAT rows |
|---|---|---|---|---|---|---:|---:|
| `ofe1_same` | byte equal | byte equal | byte equal | byte/table equal | table equal | 2192 | 2192 |
| `ofe2_same` | byte equal | byte equal | byte equal | byte/table equal | table equal | 2192 | 4384 |
| `ofe3_same` | byte equal | byte equal | byte equal | byte/table equal | table equal | 2192 | 6576 |
| `ofe4_same` | byte equal | byte equal | byte equal | byte/table equal | table equal | 2192 | 8768 |
| `ofe5_same` | byte equal | byte equal | byte equal | byte/table equal | table equal | 2192 | 10960 |
| `h2637_same` | byte equal | byte equal | byte equal | byte/table equal | table equal | 12419 | 235961 |
| `h2637_with_ui_same` | byte equal | byte equal | byte equal | byte/table equal | table equal | 12419 | 235961 |

## Runtime Notes

Baseline run times:

```text
ofe1_same           5.44s
ofe2_same           9.67s
ofe3_same          14.04s
ofe4_same          22.82s
ofe5_same          22.61s
h2637_same        674.72s
h2637_with_ui     678.99s
```

Refactored run times:

```text
ofe1_same           5.51s
ofe2_same           9.53s
ofe3_same          13.69s
ofe4_same          22.64s
ofe5_same          22.12s
h2637_same        676.56s
h2637_with_ui     673.05s
```

These timings are context for the identity run only; REFACTOR022 is not a performance package.
