# No-Compatibility Proof Checklist

Status: passed.

Static proof:

- Scanned all direct-runtime modules, including `projection.rs`, for compatibility
  request/writeback/symbol/storage tokens.
- Confirmed scheduler diff is empty.
- Confirmed R4P/Q/Z code does not construct compatibility requests, writeback
  payloads/surfaces, symbol registries, hot tables, indexed surfaces, dense
  refreshes, or dirty flushes.

Ran:

```text
rg -n "SymbolRegistry|BoundarySymbol|BoundaryValue|Option<BoundaryValue>|HillslopeWritebackSurface|KernelWritebackPayload|IndexedWritebackSurface|HotSymbolTables|HillslopeKernelRequest|execute_with_kernel|state_value_for_symbol|flux_value_for_symbol|dirty_state_ids|dirty_flux_ids" \
  crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs \
  crates/openwepp-hillslope-orchestrator/src/direct_runtime/storage.rs \
  crates/openwepp-hillslope-orchestrator/src/direct_runtime/runoff.rs \
  crates/openwepp-hillslope-orchestrator/src/direct_runtime/subsurface.rs \
  crates/openwepp-hillslope-orchestrator/src/direct_runtime/evapotranspiration.rs \
  crates/openwepp-hillslope-orchestrator/src/direct_runtime/projection.rs
```

Result: no matches (`rg` exit `1`).

```text
git diff -- crates/openwepp-hillslope-orchestrator/src/scheduler.rs
```

Result: no scheduler diff.

Runtime proof:

- Default-disabled runner fixture records zero direct-runtime counters.
- Explicit opt-in runner fixture records R4P/Q/Z phase entries and one
  production compatibility-edge handoff while public outputs remain
  compatibility-authoritative.

Ran:

```text
cargo test -p openwepp-runner r2a_ -- --nocapture
```

Result: passed, 2 tests.

```text
cargo test -p openwepp-hillslope-orchestrator direct_runtime -- --nocapture
```

Result: passed, 41 tests.
