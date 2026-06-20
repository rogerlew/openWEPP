# Implementation Test Evidence

Ran:

- `cargo test -p openwepp-hillslope-orchestrator r5b_ -- --nocapture`
  - PASS: 3 tests passed.
  - Covered direct `Normalization` compute/state/downstream/shadow behavior,
    direct `StorageBounds` compute/state/downstream/shadow behavior, explicit
    `Normalization -> StorageBounds -> DecompositionTransition` identity, anti
    alias checks, missing-upstream failure, and invalid storage/tolerance
    guards.
- `cargo test -p openwepp-hillslope-orchestrator direct_runtime -- --nocapture`
  - PASS: 45 tests passed.
  - Covered R3/R4 direct-runtime spans plus R5A lifecycle and R5B phase tests.
- `cargo test -p openwepp-runner r2a_ -- --nocapture`
  - PASS: 2 tests passed.
  - Covered default-disabled zero direct-runtime counters and explicit opt-in
    counters with R5B's additional direct day span and one compatibility-edge
    handoff.
- `cargo test --workspace`
  - PASS.

Implementation summary:

- Added `direct_runtime/normalization.rs` with R5B phase types and methods.
- Replaced the executor lifecycle's R3A input-accounting call with direct R5B
  `Normalization` and `StorageBounds` phase calls.
- Kept the R3A input-accounting span available for focused legacy tests.
- `StorageBounds` is now `Executed` in lifecycle status counts; only
  decomposition, residue, annual growth, and perennial growth remain `Hold`.
- Public output authority, scheduler phase order, default activation, output
  schemas, and compatibility publication paths are unchanged.
