# R2A Direct Type Namespace Plan

Status: queued.
Evidence mode: not run.

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
