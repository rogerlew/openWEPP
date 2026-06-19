# PERFDEEP01 Endpoint + RSS

Evidence: Ran.

## H2637 Endpoint Execution

Command run:

```bash
/usr/bin/time -f "h2637_same\t%e\t%M" \
  target/release/openwepp-cli-hill \
  --run-dir /tmp/perfho01/run-dirs/h2637 \
  --run-file /tmp/perfdeep01/runfiles/h2637_same_current.run \
  --output-dir /tmp/perfdeep01/current/h2637_same_manifest \
  --policy compat \
  --legacy-sidecar-discovery
```

Observed terminal result:

- `h2637_same 669.06 227916`
- Exit code: `0`
- Sidecar warning emitted: `MOFE01-MG-W-001` (known follow-on scope warning, non-fatal)

## Flatness Check vs PERFMIG01 Baseline

Baseline authority (`perfmig01-endpoint-timing.md`):

- Seconds: `669.97`
- Max RSS KB: `228144`

PERFDEEP01 Stage-0 measured:

- Seconds: `669.06`
- Max RSS KB: `227916`

Delta:

- Seconds: `-0.91` (`-0.14%`)
- Max RSS KB: `-228` (`-0.10%`)

Conclusion: Stage-0 scaffold remains endpoint-flat (no production hot-path regression).

## Output Identity Check

Compared anchor output directory:

- Anchor: `/tmp/perfmig01-final/current/anchor/h2637_same`
- PERFDEEP01 run: `/tmp/perfdeep01/current/h2637_same`

### Byte-identity checks (sha256)

- `H2637.hbp`: identical hash
- `H2637.wat.parquet`: identical hash
- `H2637.loss.json`: identical hash
- `H2637.plot.parquet`: identical hash

### PASS parquet semantic equality (Arrow-equivalent)

`H2637.pass.parquet` hashes differ, so semantic check was run with DuckDB:

- schema equal: `true`
- row count equal: `12419` vs `12419`
- multiset difference (`EXCEPT ALL` both directions): `0`

Conclusion: `pass.parquet` is Arrow-equivalent.

## Determinism Check (same config)

Second execution command:

```bash
/usr/bin/time -f "h2637_determinism_run2\t%e\t%M" \
  target/release/openwepp-cli-hill \
  --run-dir /tmp/perfho01/run-dirs/h2637 \
  --run-file /tmp/perfdeep01/runfiles/h2637_same_current.run \
  --output-dir /tmp/perfdeep01/determinism/h2637_same_run2 \
  --policy compat \
  --legacy-sidecar-discovery
```

Observed terminal result:

- `h2637_determinism_run2 671.10 228844`
- Exit code: `0`
- Sidecar warning emitted: `MOFE01-MG-W-001` (known follow-on scope warning, non-fatal)

Comparison baseline:

- Run1 snapshot: `/tmp/perfdeep01/determinism/run1_snapshot`
- Run2 outputs: `/tmp/perfdeep01/current/h2637_same`

Byte-comparison results:

- `H2637.hbp`: identical hash
- `H2637.wat.parquet`: identical hash
- `H2637.loss.json`: identical hash
- `H2637.plot.parquet`: identical hash
- `H2637.pass.parquet`: hash differs

`H2637.pass.parquet` semantic equality (DuckDB):

- schema equal: `true`
- row count equal: `12419` vs `12419`
- multiset difference (`EXCEPT ALL` both directions): `0`

Conclusion: same-config determinism is preserved (byte-identical primary outputs;
`pass.parquet` Arrow-equivalent).
