# Gate Results

Status: passed.

Static: final gate evidence recorded for the completed R4N write set.

Ran:

- `cargo check -p openwepp-hillslope-orchestrator` passed.
- `cargo fmt --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed.
- `cargo deny check` passed.
- `cargo test -p openwepp-hillslope-orchestrator direct_runtime -- --nocapture`
  passed: 36 tests.
- `cargo test -p openwepp-runner r2a_ -- --nocapture` passed: 2 tests.
- `cargo test -p openwepp-hillslope-orchestrator r4n -- --nocapture`
  passed: 5 tests.
- `cargo test -p openwepp-hillslope-orchestrator r4 -- --nocapture`
  passed: 26 tests.
- `cargo test -p openwepp-hillslope-orchestrator
  r2a_direct_skeleton_runs_noop -- --nocapture` passed: 1 test.
- No-compatibility forbidden-token scan across the root direct-runtime file and
  `storage.rs`, `runoff.rs`, `subsurface.rs`, and `evapotranspiration.rs`
  returned no matches.
- `git diff -- crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
  returned no scheduler diff.

Release and protected identity evidence:

```text
release_build  59.24  1121580
20baa2545fa584ffa83e06d2c1d6db55101c52c4eb3b50fa9474af9f55006389  target/release/openwepp-cli-hill
ff97ea6219cd2b5e2a600edb45e87e0e1026d8b60ea2583fd3e75b7a66ee9c92  target/release/openwepp-cli-hill.json
```

Default-disabled H2637 evidence, with direct-runtime and diagnostic
environment variables unset:

```text
r4n_h2637_default_rep1  643.84  228408
r4n_h2637_default_rep2  650.42  229204
r4n_h2637_default_rep3  649.22  228940
median                 649.22
threshold              <= 676.67
```

Each H2637 repetition emitted exactly one known warning:

```text
sidecar-warning: MOFE01-MG-W-001 EROD14 Wave-2 qin is seeded from water-transfer provenance only; true sediment-coupled qin/qout and particle-fraction handoff remains MOFE01 M-G follow-on scope.
```

DuckDB PASS row equivalence against the retained PERFDEEP07 baseline:

```text
baseline_rows  candidate_rows  left_minus_right  right_minus_left
12419          12419           0                 0
```

Candidate PASS schema count:

```text
column_count
17
```

Candidate protected output checksum:

```text
b6894a05ddde90e905313b8d90871a83497c14a0601ee7e1702905d3056fcc62  /tmp/perfmig01-final/current/anchor/h2637_same/H2637.pass.parquet
```
