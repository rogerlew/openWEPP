# R4I-L Implementation And Test Evidence

Status: complete.

Evidence class: Ran.

## Retained Implementation

- Added direct producers for:
  - liquid input;
  - runon/carry with subsurface-carry diagnostic separation;
  - cumulative infiltration plus depression-storage delta;
  - surface-saturation addback.
- Moved runoff-specific direct-runtime code to
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs`.
- Added R4A upstream completeness checks for R4I/R4J/R4K/R4L shadows before
  runoff partition can compute.
- Extended direct executor order to:

```text
R3A -> R4C -> R4D -> R4E -> R4F -> R4G -> R4I -> R4J -> R4K -> R4L -> R4A -> R4B -> R3B
```

## Focused Tests

```text
cargo test -p openwepp-hillslope-orchestrator r4il -- --nocapture
cargo test -p openwepp-hillslope-orchestrator r4 -- --nocapture
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
release_build 57.55 1097724
```

Release artifact checksums:

```text
af1be26104fe9bad388d122416abb1e868993c5c5b3b33a8d8bf834d746a86fb  target/release/openwepp-cli-hill
7827f628764d59e8c3e34b7a5d25b280d1c6f33ff594fb97f3030e26ab95477c  target/release/openwepp-cli-hill.json
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
