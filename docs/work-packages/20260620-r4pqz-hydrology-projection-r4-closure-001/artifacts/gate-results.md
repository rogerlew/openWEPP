# Gate Results

Status: passed.

Static: final gate evidence recorded for the completed R4P/Q/Z write set.

Ran:

- `cargo check -p openwepp-hillslope-orchestrator` passed.
- `cargo fmt --check` passed.
- `cargo clippy --workspace --all-targets -- -D warnings` passed.
- `cargo test --workspace` passed.
- `cargo deny check` passed.
- `cargo test -p openwepp-hillslope-orchestrator r4pqz -- --nocapture`
  passed: 5 tests.
- `cargo test -p openwepp-hillslope-orchestrator direct_runtime --
  --nocapture` passed: 41 tests.
- `cargo test -p openwepp-hillslope-orchestrator r4 -- --nocapture`
  passed: 31 tests.
- `cargo test -p openwepp-runner r2a_ -- --nocapture` passed: 2 tests.
- No-compatibility forbidden-token scan across the root direct-runtime file and
  `storage.rs`, `runoff.rs`, `subsurface.rs`, `evapotranspiration.rs`, and
  `projection.rs` returned no matches.
- `git diff -- crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
  returned no scheduler diff.
- Scoped `markdown-doc lint` passed for `docs/ROADMAP.md`,
  `docs/work-packages/README.md`,
  `docs/work-packages/r4-burndown-execplan.md`, and the R4P/Q/Z package
  directory.
- `git diff --check` passed.

Release and protected identity evidence:

```text
release_build  58.64  1127024
15d1fb25167a9efe850ae23be7cfcbc4101b743d03128fbaea3b453a1c6a0b5f  target/release/openwepp-cli-hill
28cdca1f1e11dd9eb70546b48998c1d4e49ca38b484ac9e787530367aed4bf19  target/release/openwepp-cli-hill.json
```

Default-disabled H2637 evidence, with direct-runtime and diagnostic
environment variables unset:

```text
r4pqz_h2637_default_rep1  645.54  227408
r4pqz_h2637_default_rep2  644.74  228796
r4pqz_h2637_default_rep3  640.28  229216
median                   644.74
threshold                <= 676.67
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
43ece8ca3c539a4dfa21b9f569786d5b05b4281e9e2ba45fb6e9fb087e06b9c2  /tmp/perfmig01-final/current/anchor/h2637_same/H2637.pass.parquet
```
