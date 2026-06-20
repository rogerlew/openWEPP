# R3A No-Compatibility Proof Checklist

Status: complete.
Evidence mode: Static + Ran.

Ran:

```text
rg -n "execute_with_kernel|HillslopeKernelRequest|KernelWritebackPayload|HillslopeWritebackSurface|state_value_for_symbol|flux_value_for_symbol|SymbolRegistry|HotSymbolTables|IndexedWritebackSurface|dense|dirty|build_registry_for_run" crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs
```

Result: no matches, exit code `1`.

Ran:

```text
git diff -- crates/openwepp-hillslope-orchestrator/src/scheduler.rs
```

Result: no diff.

| Surface | Result | Evidence |
|---|---|---|
| `execute_with_kernel*` | PASS | Source scan no matches. |
| `HillslopeKernelRequest` | PASS | Source scan no matches. |
| `KernelWritebackPayload` | PASS | Source scan no matches. |
| `HillslopeWritebackSurface` | PASS | Source scan no matches. |
| `state_value_for_symbol` / `flux_value_for_symbol` | PASS | Source scan no matches. |
| `SymbolRegistry` / `SymbolRegistry::id_of` | PASS | Source scan no matches. |
| `HotSymbolTables` | PASS | Source scan no matches. |
| `IndexedWritebackSurface` | PASS | Source scan no matches. |
| dense refresh / dirty flush | PASS | Source scan no matches for `dense` or `dirty`. |
| owned legacy-symbol construction in direct execution | PASS | No boundary-symbol or registry tokens in `direct_runtime.rs`; source-token test passes. |
| direct-span compatibility edge counter | PASS | R3A span reports zero edge invocations; runner explicit opt-in records one production compatibility handoff after direct execution. |

Runtime proof:

- Default-disabled runner fixture asserts all direct construction, phase-span,
  compute, mutation, downstream, shadow, and compatibility-edge counters are
  zero.
- Explicit opt-in runner fixture asserts span counters become positive and
  exactly one compatibility-edge handoff is recorded after direct execution
  returns to compatibility publication.
- Orchestrator R3A tests assert exact per-span counter deltas.

Disposition: PASS.
