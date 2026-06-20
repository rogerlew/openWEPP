# R4D No-Compatibility Proof Checklist

Status: complete.
Evidence mode: Static + Ran.

Required checks:

| Check | Result | Evidence |
|---|---|---|
| Direct runtime forbidden-token source scan | PASS | `rg -n "SymbolRegistry|BoundarySymbol|BoundaryValue|Option<BoundaryValue>|HillslopeWritebackSurface|KernelWritebackPayload|IndexedWritebackSurface|HotSymbolTables|HillslopeKernelRequest|execute_with_kernel|state_value_for_symbol|flux_value_for_symbol|dirty_state_ids|dirty_flux_ids" crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs` returned no matches. |
| Scheduler no-diff check | PASS | `git diff -- crates/openwepp-hillslope-orchestrator/src/scheduler.rs` returned no diff. |
| Default-disabled direct counters are zero | PASS | `cargo test -p openwepp-runner r2a_ -- --nocapture`; default fixture passed. |
| Explicit opt-in direct counters are positive | PASS | Runner opt-in fixture now requires R3A/R3B/R3C/R4A/R4B/R4C/R4D counters and one production compatibility handoff. |
| Direct span compatibility-edge count remains zero | PASS | R4D focused test, R4B focused test, and aggregate R2A direct test assert direct span compatibility-edge invocation count remains zero. |
