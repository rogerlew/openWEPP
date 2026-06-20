# R3A No-Compatibility Proof Checklist

Status: queued.
Evidence mode: not run.

Forbidden in direct phase-span execution unless explicitly listed as edge-only:

| Surface | Result | Evidence |
|---|---|---|
| `execute_with_kernel*` | NOT RUN | queued |
| `HillslopeKernelRequest` | NOT RUN | queued |
| `KernelWritebackPayload` | NOT RUN | queued |
| `HillslopeWritebackSurface` | NOT RUN | queued |
| `state_value_for_symbol` / `flux_value_for_symbol` | NOT RUN | queued |
| `SymbolRegistry` / `SymbolRegistry::id_of` | NOT RUN | queued |
| `HotSymbolTables` | NOT RUN | queued |
| `IndexedWritebackSurface` | NOT RUN | queued |
| dense refresh / dirty flush | NOT RUN | queued |
| owned legacy-symbol construction in direct execution | NOT RUN | queued |
| direct-span compatibility edge counter | NOT RUN | queued |

Any `FAIL`, `BLOCKED`, or unjustified `NOT RUN` prevents completion.

The proof must combine static call-graph/source scans with runtime counters.
