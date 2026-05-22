# PL03 Scheduler Ordering Compliance

Status: `complete`
Evidence mode: `Static + Ran`

Static:
- PL seam requirements require deterministic precondition signaling for decomposition/growth/water-balance phase ordering.

Ran:
- Projected explicit ordering precondition symbols into schedule surface for scheduler consumption.

## Implemented Ordering Preconditions

1. `pl_order_decomp_before_soil = 1`
2. `pl_order_growth_after_decomp = 1`
3. `pl_order_watbal_after_growth = 1`

## Compliance Notes

1. Preconditions are projected unconditionally in PL management adaptation, independent of slot content.
2. Preconditions are carried into merged state surface via `merged_state_surface()` and `build_hillslope_runtime_surface_from_management`.
3. Positive integration test asserts presence of ordering symbol in merged runtime surface.

## Evidence Links

- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:764`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:768`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:772`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:685`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:1119`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs:2249`
- `/home/workdir/openWEPP/docs/work-packages/20260522-pl02-plant-runtime-boundary-contract-001/artifacts/pl-runtime-seam-requirements.md`
