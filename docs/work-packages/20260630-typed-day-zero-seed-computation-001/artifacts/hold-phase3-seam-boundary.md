# HOLD Phase 3 Seam Boundary

Evidence mode: Static/Ran.

## Result

`EXECUTED-HOLD-PHASE3-SEAM-BOUNDARY`.

## What Completed

Ran:

- Production direct setup no longer constructs the static
  `HillslopeWritebackSurface` seed authority, persistent lane symbol-map state,
  symbol registry, or hot symbol tables.
- Production direct execution and snowbench diagnostics construct typed
  `DirectProductionSeedAuthority` from parsed inputs, sidecars, and day-one
  climate.
- Obsolete direct day-zero seed-surface bridge functions were deleted.
- H2637 protected outputs remain byte-identical; manifest
  `direct_runtime_counters.compatibility_edge_invocations=0`.
- Full gates passed, including `cargo nextest run --workspace --profile full`
  (`1880` passed, `1` skipped).

## Hold Boundary

Static:

ADR-0030 explicitly retains the deprecated `--compatibility-runtime`
replay/comparator seam. The remaining symbol-map runtime files and carrier
types are the implementation of that seam and of legacy scheduler tests:

- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`;
- `crates/openwepp-hillslope-orchestrator/src/day_frame.rs`;
- `HillslopeWritebackSurface`, `HillslopeKernelRequest`,
  `KernelWritebackPayload`, and `SymbolRegistry` carrier paths.

Deleting those files now would either remove the explicit seam or require a new
replacement seam. That decision is outside this package's binding non-scope
(`Do not delete the explicit --compatibility-runtime replay seam`).

## Required Decision To Continue

Choose one before a broader file deletion package:

- remove the explicit `--compatibility-runtime` seam entirely; or
- replace it with a direct-native replay/comparator that does not depend on the
  symbol-map scheduler.

Until then, the production direct path is single-authority at setup and hot
loop, while the retained symbol-map runtime is diagnostic/test-only.
