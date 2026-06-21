# Process Span Contract

Status: complete.
Evidence mode: Static.

R5E canonical phases are exactly `DirectPhaseKind::ORDERED`:

1. `Normalization`
2. `StorageBounds`
3. `DecompositionTransition`
4. `ResiduePartitionTransition`
5. `AnnualGrowthTransition`
6. `PerennialGrowthTransition`
7. `PercolationDeepSeepage`
8. `Evapotranspiration`
9. `Drainage`
10. `LateralTransfer`
11. `PlantRootUptake`
12. `RunoffReconciliation`
13. `StorageReconciliation`
14. `ClosureDiagnostics`

Each direct day/lane frame must record exactly one canonical entry for each
phase in this order. R4 and R5 direct spans may record direct sub-operation
counters, but those counters are not canonical phase executions.

Current implementation delta:

- `DirectExecutionReport::canonical_phase_entry_count` records the canonical
  phase entries separately from `direct_phase_entry_count`.
- Focused R5E test asserts canonical entries equal
  `14 * lane_count * day_count`.
- Focused R5E test also asserts direct sub-operation counters remain greater
  than canonical entries, proving folded R4/R5 spans are not being counted as
  duplicate canonical phase executions.
