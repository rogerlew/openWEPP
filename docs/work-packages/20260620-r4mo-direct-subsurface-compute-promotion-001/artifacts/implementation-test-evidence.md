# Implementation Test Evidence

Status: complete.

Evidence class: Ran.

## Retained Implementation

- Added `direct_runtime/subsurface.rs` for request-free direct WB18/WB19
  compute from typed layer vectors.
- Added R4M percolation span:
  `PercolationDeepSeepage -> StorageReconciliation`.
- Added R4O subsurface span:
  `Drainage -> LateralTransfer -> StorageReconciliation`.
- R4M mutates direct layer storage, computes `D`, `Pe`, per-layer percolation
  fluxes, feeds R4B `deep_seepage_m`, produces downstream operands, and
  shadow-projects the WB18 result.
- R4O consumes R4M direct percolation state, computes drainage `Qdd`, lateral
  `q`, final `Qd`, carry arrays, layer withdrawals, feeds R4B
  `subsurface_loss_m`, produces downstream operands, and shadow-projects the
  WB19 result.
- R4B now fails closed unless the R4M percolation and R4O subsurface-compute
  shadows exist.
- The aggregate direct executor order now runs:

```text
R3A -> R4C -> R4M -> R4O -> R4F -> R4G -> R4I -> R4J -> R4K -> R4L -> R4A -> R4B -> R3B
```

## Focused Tests

```text
cargo test -p openwepp-hillslope-orchestrator r4mo -- --nocapture
cargo test -p openwepp-hillslope-orchestrator direct_runtime -- --nocapture
cargo test -p openwepp-hillslope-orchestrator r4 -- --nocapture
cargo test -p openwepp-hillslope-orchestrator r2a_direct_skeleton_runs_noop -- --nocapture
cargo test -p openwepp-runner r2a_ -- --nocapture
```

Result: all focused tests passed.

Focused coverage includes daily and hourly-restrictive WB18 parity against the
compatibility kernel, hourly WB19 drainage-before-lateral ordering, realized
withdrawal/carry-array production, `q + Qdd = Qd`, invalid-domain guards,
missing-upstream guards, and anti-alias fixtures for `D`, `Pe`, `q`, `Qdd`,
and `Qd`.

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
release_build 58.46 1111080
```

Release artifact checksums:

```text
0b24de3fe101292cb39a48c8f2bb103629906c4fe311cae9937b966d4c6f676d  target/release/openwepp-cli-hill
641db61f7fb817badd0bda639e8013db1513c2103daf1fbdcb8f8f732d9c2874  target/release/openwepp-cli-hill.json
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

Package-local run manifests:

```text
2da6809b2f29da888a42a81a9170a1d61d130e24789accb1d88a5bcb65d0266f  /tmp/r4mo-h2637/default/rep1/h2637_same/openwepp_hillslope_run_manifest.json
6d57cbc115f5d60db6f9ad6fc45560e730b6cd8743f52ea5cab88ab43287bff1  /tmp/r4mo-h2637/default/rep2/h2637_same/openwepp_hillslope_run_manifest.json
9d0df9c00a293b877261f4558d3a7dc2c7ae5d1379003685c0aac877b57662db  /tmp/r4mo-h2637/default/rep3/h2637_same/openwepp_hillslope_run_manifest.json
74ccd856e6100799251e8a7ce2ec377f97057848a0280dbe1254a17b00a967f8  /tmp/perfmig01-final/current/anchor/h2637_same/H2637.pass.parquet
```

Result: PASS row identity is preserved.
