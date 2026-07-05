# Local CI Timing Tools

`nextest_timing.py` records local nextest timing evidence under
`target/local-ci-history/` (already ignored by git). Use it when a gate run is
expensive enough that the wall time and slow-test list should be durable.

## Common Commands

Record an existing JUnit file:

```bash
python tools/local_ci/nextest_timing.py summarize \
  --label full-existing \
  --profile full
```

Run a gate and record it:

```bash
python tools/local_ci/nextest_timing.py run \
  --label quick \
  --profile quick \
  -- cargo nextest run --workspace --profile quick
```

`run` and `sweep` delete the selected JUnit file before executing and require a
fresh JUnit file afterward. Use `summarize` when intentionally recording an
existing JUnit file. When `--junit` is omitted, the path defaults to
`target/nextest/<profile>/junit.xml`.

Benchmark a nextest test-group cap without editing the committed config:

```bash
python tools/local_ci/nextest_timing.py sweep \
  --group cli-fixture \
  --caps 2,3 \
  --profile full \
  --filterset 'binary(/^(cli01_runner_hillslope_integration|cli03_runner_contract_derived_tests|cli04_runner_wat_parquet_contract_derived_tests)$/)'
```

The latest summary is written to `target/local-ci-history/latest.md`; the full
append-only log is `target/local-ci-history/nextest-runs.jsonl`.
