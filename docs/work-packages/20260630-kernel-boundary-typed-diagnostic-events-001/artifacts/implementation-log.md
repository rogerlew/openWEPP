# Implementation Log

Evidence class: Static plus focused compile.

## Code Changes

Added `crates/openwepp-hillslope-orchestrator/src/direct_runtime/diagnostic_events.rs`
with typed event payloads:

- `DirectRunoffRebalanceTraceEvent`
- `DirectEvapotranspirationTraceEvent`
- `DirectPercolationTraceEvent`
- `DirectSubsurfaceSaturationTraceEvent`

Repointed direct-runtime trace writers:

- `direct_runtime/runoff.rs`
  - `maybe_write_r7h_runoff_rebalance_trace` now consumes
    `DirectRunoffRebalanceTraceEvent`.
- `direct_runtime/evapotranspiration.rs`
  - `maybe_write_r7h_et_trace` now consumes
    `DirectEvapotranspirationTraceEvent`.
- `direct_runtime/subsurface.rs`
  - percolation and saturation trace writers now consume typed event payloads.

The new payloads are direct-runtime typed state. They are not wrappers around
`HillslopeKernelRequest`, `HillslopeWritebackSurface`, `KernelWritebackPayload`,
`SymbolRegistry`, or `BoundarySymbol` lookup.

## Hold Finding

This implementation does not reduce the requested TRACE-class carrier-reference
count. The requested files still need their own typed event sources:

- HPHYS rows need direct-runtime publication/diagnostic event data rather than
  scheduler-surface scans.
- Shadow/audit diagnostics need typed guard/source-scan replacements.
- Frame-roundtrip diagnostics are scheduler/symbol-surface diagnostics and
  should be deleted or replaced by typed-frame tests.
- The remaining `frost_entry.rs` request references are mixed with active-frost
  kernel-boundary readers.
