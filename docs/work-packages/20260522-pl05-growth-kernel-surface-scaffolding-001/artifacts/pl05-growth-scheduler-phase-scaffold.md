# PL05 Growth Scheduler Phase Scaffold

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- Canonical hillslope scheduler order now includes two growth placeholder phases.
- Deterministic dependency edges encode growth-before-watbal ordering semantics.

Ran:
- Scheduler graph/topology tests and integration suites passed under `cargo test --workspace`.

## Canonical Phase Order (PL05)

1. `Normalization`
2. `StorageBounds`
3. `AnnualGrowthTransition`
4. `PerennialGrowthTransition`
5. `Evapotranspiration`
6. `PercolationDeepSeepage`
7. `LateralTransfer`
8. `Drainage`
9. `RunoffReconciliation`
10. `StorageReconciliation`
11. `ClosureDiagnostics`

## Deterministic Edge Scaffold

- `StorageBounds -> AnnualGrowthTransition`
- `AnnualGrowthTransition -> PerennialGrowthTransition`
- `PerennialGrowthTransition -> Evapotranspiration`

## Placeholder Dispatch Semantics

1. Growth phases are always present in scheduler order.
2. Growth runtime activation sentinel:
- `pl_schedule_slot_count`
3. Branch activation uses first-slot management class seed:
- `pl_growth_slot_0001_crop_0001_imngmt`
4. Branch-mismatched growth phase emits nominal pass-through request (`growth_context=None`).

## Evidence Links

- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:21`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:45`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:60`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:665`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/lib.rs:1054`
