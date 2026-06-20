# Implementation And Test Evidence

Status: complete.
Evidence mode: Ran.

## Implementation Summary

Implemented R4A as a direct lane/day process span in
`crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`.

Production changes:

- added `DIRECT_R4A_RUNOFF_PARTITION_SPAN =
  [RunoffReconciliation, StorageReconciliation, ClosureDiagnostics]`;
- added `DirectRunoffPartitionInputs`;
- added `DirectRunoffPartitionState`;
- added `DirectRunoffDownstreamOperands`;
- added `DirectRunoffShadowProjection`;
- added `DirectRunoffPartitionSpanReport`;
- added `DirectDayFrame::run_r4a_runoff_partition_span`;
- validates finite/nonnegative direct operands and fails closed on overdrawn
  partition runoff or overflowed derived runoff;
- mutates only direct runtime state:
  `DirectWaterState::infiltration_m`, `DirectWaterState::runoff_m`, and direct
  runoff partition state/downstream/shadow surfaces;
- updated `DirectFrameExecutor::run_skeleton` to run R4A after R3A and before
  R3B for each seeded direct lane/day frame;
- exported R4A constants and public types from
  `crates/openwepp-hillslope-orchestrator/src/lib.rs`;
- updated runner default/opt-in direct-runtime counter assertions.

R4A remains direct-only and no-publication. It does not migrate full WB12/WB14,
Green-Ampt infiltration solving, scheduler execution, output schemas, or default
activation.

## Focused Tests

Ran:

- `cargo test -p openwepp-hillslope-orchestrator r4a_ -- --nocapture`
  - PASS: `2 passed; 0 failed`.
- `cargo test -p openwepp-hillslope-orchestrator r2a_direct -- --nocapture`
  - PASS: `3 passed; 0 failed`.
- `cargo test -p openwepp-runner r2a_ -- --nocapture`
  - PASS: `2 passed; 0 failed`.

Focused test coverage added:

- exact R4A phase-span identity;
- exact binary-fraction runoff-partition process result;
- direct water-state mutation for infiltration and runoff;
- downstream operands and shadow projection;
- anti-alias checks separating accepted `q_runoff_m` from precipitation-only,
  no-depression-storage, no-saturation-addback, and infiltration-as-runoff
  candidates;
- fail-closed nonfinite liquid input;
- fail-closed negative depression-storage delta;
- fail-closed overdrawn partition runoff;
- fail-closed overflowed final runoff.

## Full Gates

Ran:

- `cargo fmt --check`: PASS.
- `cargo clippy --workspace --all-targets -- -D warnings`: PASS.
- `cargo test --workspace`: PASS.
- `cargo deny check`: PASS, `advisories ok, bans ok, licenses ok, sources ok`.

## Release Build

Ran:

- `/usr/bin/time -f 'release_build\t%e\t%M' cargo build --release -p openwepp-runner --bin openwepp-cli-hill`

Result:

- PASS.
- Build time: `57.96 s`.
- Max RSS: `1088060 KB`.
- Release binary SHA-256:
  `3ff8b1ad6658f0a69b43025d4ba81839eb4be8fa938b9e71469abfdad1002455`.
- Final release sidecar SHA-256:
  `7297bd5bf0ba0896a6e4b380d8167066c1f010f21bea1a000cf8b590f700cccf`.
