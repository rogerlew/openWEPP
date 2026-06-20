# R4E-H Implementation And Test Evidence

Status: complete.

Evidence class: Ran.

## Retained Implementation

- Added direct state/input/downstream/shadow/span report types for:
  - subsurface-loss handoff;
  - aggregate evapotranspiration handoff;
  - signed snow-coupling handoff.
- Added producer methods in `direct_runtime/storage.rs` for R4E, R4F, and
  R4G.
- Mutated R4B storage-reconciliation inputs from producer outputs instead of
  accepting manually seeded values.
- Extended direct executor order to:

```text
R3A -> R4C -> R4D -> R4E -> R4F -> R4G -> R4A -> R4B -> R3B
```

## Focused Tests

```text
cargo test -p openwepp-hillslope-orchestrator r4 -- --nocapture
cargo test -p openwepp-hillslope-orchestrator r4e -- --nocapture
cargo test -p openwepp-hillslope-orchestrator r4f -- --nocapture
cargo test -p openwepp-hillslope-orchestrator r4g -- --nocapture
cargo test -p openwepp-hillslope-orchestrator r2a_direct_skeleton_runs_noop -- --nocapture
cargo test -p openwepp-runner r2a_ -- --nocapture
```

Result: all focused tests passed.

## Workspace Gates

```text
cargo fmt --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo deny check
```

Result: all workspace gates passed.

## H2637 Identity Evidence

Release build:

```text
/usr/bin/time -f 'release_build\t%e\t%M' cargo build --release -p openwepp-runner --bin openwepp-cli-hill
release_build 57.22 1103236
```

Release artifact checksums:

```text
b93711c11c939263ac0a19e27f0221c798343bc85d2613d096ad5257e9c9e1f8  target/release/openwepp-cli-hill
33f254cb1921277c53073e3bcd31c9e6e7f28d6e71b553d5bb4dff538eeb73cd  target/release/openwepp-cli-hill.json
```

H2637 PASS row equivalence used the manifest output location
`/tmp/perfmig01-final/current/anchor/h2637_same/H2637.pass.parquet` and the
PERFDEEP07 default-disabled baseline
`/tmp/perfdeep07/default/rep1/h2637_same/H2637.pass.parquet`.

```text
baseline_rows  candidate_rows  left_minus_right  right_minus_left
12419          12419           0                 0

column_count
17
```

Result: PASS row identity is preserved.
