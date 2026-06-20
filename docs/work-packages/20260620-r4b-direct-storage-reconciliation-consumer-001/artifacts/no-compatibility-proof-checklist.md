# R4B No-Compatibility Proof Checklist

Status: complete.
Evidence mode: Ran.

Checks:

| Check | Result | Evidence |
|---|---|---|
| Direct runtime forbidden-token source scan | PASS | `rg -n "SymbolRegistry|BoundarySymbol|BoundaryValue|Option<BoundaryValue>|HillslopeWritebackSurface|KernelWritebackPayload|IndexedWritebackSurface|HotSymbolTables|HillslopeKernelRequest|execute_with_kernel|state_value_for_symbol|flux_value_for_symbol|dirty_state_ids|dirty_flux_ids" crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs` returned no matches. |
| Scheduler no-diff check | PASS | `git diff -- crates/openwepp-hillslope-orchestrator/src/scheduler.rs` produced no diff. |
| Default-disabled direct counters are zero | PASS | `cargo test -p openwepp-runner r2a_ -- --nocapture` passed the default-disabled zero-counter fixture. |
| Explicit opt-in direct counters are positive | PASS | `cargo test -p openwepp-runner r2a_ -- --nocapture` passed opt-in assertions covering R3A/R3B/R3C/R4A/R4B counters. |
| Direct span compatibility-edge count remains zero | PASS | `cargo test -p openwepp-hillslope-orchestrator r4b_ -- --nocapture` and aggregate direct-runtime report assertions passed. |
