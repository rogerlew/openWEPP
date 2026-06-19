# PERFMIG01 Migration

Evidence: Static + Ran.

## Flipped Authority

PERFMIG01 ratified ADR-0023 and added the first production id-backed writeback
surface for the WB11 warm-rain runoff branch.

Static:

- `KernelRunResponse` now carries an optional
  `IndexedKernelWritebackPayload`; `KernelRunResponse::with_indexed_writeback`
  returns an empty logical payload and the id-backed payload.
- The scheduler detects `response.indexed_writeback`, evaluates the id-backed
  payload, applies it to `IndexedWritebackSurface`, and materializes only the
  updated ids back to logical maps for unmigrated downstream phases.
- `Wb11HydrologyKernel::run_runoff_reconciliation` now tries the migrated
  id-backed writeback before constructing the legacy logical payload.

## Migrated Branch

The migrated branch is the inactive-snow / inactive-frost / no-irrigation /
no-MOFE-hourly-carry warm-rain WB11 runoff path. It emits:

| Payload family | Count | Authority |
| --- | ---: | --- |
| State updates | 543 | `SymbolId` via `IndexedWritebackField` |
| Flux updates | 8 | `SymbolId` via `IndexedWritebackField` |

The id-backed payload contains the same WB12/WB14/irrigation-runtime/snow-runtime
scalar outputs and 24-hour inactive snow diagnostic series as the legacy logical
writeback path.

## Named Logical Boundaries

The production phase still has branches that this rung intentionally leaves
logical, with a named boundary:

- active snow coupling;
- active frost coupling;
- active irrigation event;
- MOFE hourly carry arrays.

Those branches return `Ok(None)` from
`build_warm_rain_runoff_indexed_writeback` and fall through to the legacy
`KernelWritebackPayload::with_updates` path. This is not a silent fallback for
the migrated branch: once the warm-rain branch is selected, missing required id
symbols fail closed through the existing missing-required-symbol guard class.

## Transition Boundary

The transition boundary is scheduler-side compatibility materialization:

1. evaluate id-backed finite/domain constraints;
2. apply id values to `IndexedWritebackSurface`;
3. resolve only the updated ids through the registry;
4. insert those updated symbols into the logical state/flux maps for downstream
   unmigrated phases.

This preserves the public logical surfaces while moving the migrated kernel
writeback success path away from logical-map authority.

## Key Files

- `docs/decisions/0023-array-authoritative-hot-path-state.md`
- `crates/openwepp-kernel-contract/src/lib_mod/core_types/02_boundary_values_and_kernel_requests.rs`
- `crates/openwepp-kernel-contract/src/lib_mod/writeback.rs`
- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/kernel_phases_mod/hydrology_phase_runoff_reconciliation.rs`
