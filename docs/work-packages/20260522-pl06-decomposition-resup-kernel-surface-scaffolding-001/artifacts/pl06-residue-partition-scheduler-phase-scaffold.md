# PL06 Residue Partition Scheduler Phase Scaffold

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Canonical hillslope scheduler order now includes explicit decomposition and residue-partition placeholder phases.
- Deterministic dependency edges encode decomposition/residue before growth and preserve growth-before-watbal entry constraints.

Ran:
- Scheduler graph/topology tests and integration suites passed under `cargo test --workspace`.

## Canonical Phase Order (PL06)

1. `Normalization`
2. `StorageBounds`
3. `DecompositionTransition`
4. `ResiduePartitionTransition`
5. `AnnualGrowthTransition`
6. `PerennialGrowthTransition`
7. `Evapotranspiration`
8. `PercolationDeepSeepage`
9. `LateralTransfer`
10. `Drainage`
11. `RunoffReconciliation`
12. `StorageReconciliation`
13. `ClosureDiagnostics`

## Deterministic Edge Scaffold

- `StorageBounds -> DecompositionTransition`
- `DecompositionTransition -> ResiduePartitionTransition`
- `ResiduePartitionTransition -> AnnualGrowthTransition`
- `AnnualGrowthTransition -> PerennialGrowthTransition`
- `PerennialGrowthTransition -> Evapotranspiration`

## Placeholder Dispatch Semantics

1. Decomposition and residue phases are always present in scheduler order.
2. Transition runtime activation sentinel:
- `pl_schedule_slot_count`
3. Branch activation uses first-slot management class seed:
- `pl_growth_slot_0001_crop_0001_imngmt`
4. Branch-mismatched growth phase emits nominal pass-through request (`growth_context=None`); decomposition phases still validate required ordering/seed state for active management class.

## Evidence Links

- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:22`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:72`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:981`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:532`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:610`
