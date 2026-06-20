# Implementation And Test Evidence

Status: complete.
Evidence mode: Ran.

## Implementation Summary

Implemented R3C as a run-level direct span in
`crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`.

Production changes:

- added `DIRECT_R3C_LANE_TRANSFER_SPAN =
  [LateralTransfer, RunoffReconciliation, ClosureDiagnostics]`;
- added direct run-level lane transfer ledger state to `DirectRunFrame`;
- added `DirectLaneTransferLedger`;
- added `DirectRunTransferDownstreamOperands`;
- added `DirectRunTransferShadowProjection`;
- added `DirectRunTransferSpanReport`;
- added typed topology/domain errors for frame lane-count mismatch, invalid
  lane topology, and invalid outlet count;
- added `DirectRunFrame::run_r3c_lane_transfer_span`;
- validates lane ids, upstream/downstream range, reciprocal upstream/downstream
  links, exactly one outlet, finite/nonnegative area metadata, finite/
  nonnegative transfer arrays, and finite derived transfer totals;
- updated `DirectFrameExecutor::run_skeleton` to run R3C once per direct run,
  then R3A/R3B once per lane;
- exported R3C constants and public types from
  `crates/openwepp-hillslope-orchestrator/src/lib.rs`;
- updated runner default/opt-in direct-runtime counter assertions.

R3C remains diagnostic-only. It does not migrate process equations, publish
outputs, cut over compatibility runtime paths, or activate direct mode by
default.

## Focused Tests

Ran:

- `cargo test -p openwepp-hillslope-orchestrator r3c_ -- --nocapture`
  - PASS: `2 passed; 0 failed`.
- `cargo test -p openwepp-hillslope-orchestrator r2a_direct -- --nocapture`
  - PASS: `3 passed; 0 failed`.
- `cargo test -p openwepp-runner r2a_ -- --nocapture`
  - PASS: `2 passed; 0 failed`.

Focused test coverage added:

- exact R3C phase-span identity;
- exact binary-fraction multi-lane transfer ledger projection;
- exact run-level downstream operands and shadow projection;
- deterministic aggregate counters for R3C plus per-lane R3A/R3B;
- fail-closed negative upstream-area ratio;
- fail-closed invalid downstream lane id;
- fail-closed nonreciprocal topology;
- fail-closed multiple outlet topology;
- fail-closed transfer sum overflow;
- fail-closed received-transfer overflow.

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
- Build time: `57.73 s`.
- Max RSS: `1104412 KB`.
- Release binary SHA-256:
  `ad4df663c8b34f907b2e3e54bd4c695d5d175a2f6497385d4af01b63d404c671`.
- Final release sidecar SHA-256:
  `e06094508cbdbe948a28fc0728689d66286ed866ad3b00d4b2a4257dfc585c8e`.
