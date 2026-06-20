# R2A Direct Type Namespace Plan

Status: complete.
Evidence mode: Static + Ran.

Required direct type family:

- `DirectRunFrame`
- `DirectLaneFrame`
- `DirectDayFrame`
- `DirectPublicationFrame`
- `DirectPhaseView`

Prohibited direct-frame storage:

- `SymbolRegistry`
- `BoundarySymbol`
- `BoundaryValue`
- `Option<BoundaryValue>`
- `HillslopeWritebackSurface`
- `KernelWritebackPayload`
- `IndexedWritebackSurface`
- `HotSymbolTables`
- logical/dense refresh state
- dirty-flush state

Execution must record source paths, public API surface, and static proof that
the prohibited types do not appear in direct-frame storage.

## Implemented Surface

Static:

- Direct namespace path:
  `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`.
- Public frame and execution API:
  `DirectRunIdentity`, `DirectRunFrame`, `DirectLaneFrame`,
  `DirectDayFrame`, `DirectPublicationFrame`, `DirectPhaseView`,
  `DirectPhaseKind`, `DirectPhasePlan`, `DirectWaterState`,
  `DirectDayForcing`, `DirectTransferBuffers`, `DirectFrameExecutor`,
  `DirectExecutorMode`, `DirectExecutionReport`,
  `DirectRuntimeAuditSnapshot`, `direct_runtime_audit_snapshot`,
  `reset_direct_runtime_audit_counters`, and `DirectRuntimeError`.
- `DirectRunIdentity::new` fails closed on zero lane or day count.
- `DirectFrameExecutor::run_skeleton` walks one seed day per lane and the
  14 planned direct phases, but performs no phase math and publishes no
  production outputs.

Ran:

- `rg -n "execute_with_kernel|HillslopeKernelRequest|KernelWritebackPayload|HillslopeWritebackSurface|state_value_for_symbol|flux_value_for_symbol|SymbolRegistry|HotSymbolTables|IndexedWritebackSurface|dense|dirty|build_registry_for_run" crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`
  returned no matches.
- `cargo test -p openwepp-hillslope-orchestrator r2a_direct -- --nocapture`
  passed: 3 tests.
