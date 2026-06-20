# No-Compatibility Proof Checklist

Status: passed.

Static checks:

- Forbidden-token scan ran across direct-runtime files, including the new R4N
  module, and returned no matches.
- Scheduler diff check returned no scheduler diff.
- R4N code constructs no compatibility request, writeback surface, symbol
  registry, hot table, indexed surface, dense refresh, or dirty flush.

Runtime checks:

- Default-disabled runner fixture records zero direct-runtime counters.
- Explicit opt-in runner fixture records positive R4N phase entries and one
  production compatibility-edge handoff while public output remains
  compatibility-authoritative.

Ran:

```text
rg -n "SymbolRegistry|BoundarySymbol|BoundaryValue|Option<BoundaryValue>|HillslopeWritebackSurface|KernelWritebackPayload|IndexedWritebackSurface|HotSymbolTables|HillslopeKernelRequest|execute_with_kernel|state_value_for_symbol|flux_value_for_symbol|dirty_state_ids|dirty_flux_ids" \
  crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs \
  crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs \
  crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs \
  crates/openwepp-hillslope-orchestrator/src/direct_runtime/subsurface.rs \
  crates/openwepp-hillslope-orchestrator/src/direct_runtime/evapotranspiration.rs
```

Result: no matches.

```text
git diff -- crates/openwepp-hillslope-orchestrator/src/scheduler.rs
```

Result: no output.
