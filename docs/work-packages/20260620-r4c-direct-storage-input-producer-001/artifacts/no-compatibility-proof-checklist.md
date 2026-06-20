# R4C No-Compatibility Proof Checklist

Status: complete.
Evidence mode: Static + Ran.

## Final Checks

| Check | Result | Evidence |
|---|---|---|
| Direct runtime forbidden-token source scan | PASS | `rg` over `direct_runtime.rs` and `direct_runtime/storage.rs` for compatibility storage/request/writeback/symbol APIs returned no matches. |
| Scheduler no-diff check | PASS | `git diff -- crates/openwepp-hillslope-orchestrator/src/scheduler.rs` returned no diff. |
| Default-disabled direct counters are zero | PASS | `cargo test -p openwepp-runner r2a_ -- --nocapture`; default fixture preserves no direct skeleton construction. |
| Explicit opt-in direct counters are positive | PASS | Runner opt-in fixture now requires R3A/R3B/R3C/R4A/R4B/R4C counters and one compatibility-edge handoff. |
| Direct span compatibility-edge count remains zero | PASS | Focused R4C/R4B tests assert per-span `compatibility_edge_invocation_count == 0`; aggregate direct runtime report remains zero. |

Forbidden source tokens scanned:

```text
SymbolRegistry BoundarySymbol BoundaryValue Option<BoundaryValue>
HillslopeWritebackSurface KernelWritebackPayload IndexedWritebackSurface
HotSymbolTables HillslopeKernelRequest execute_with_kernel
state_value_for_symbol flux_value_for_symbol dirty_state_ids dirty_flux_ids
```
