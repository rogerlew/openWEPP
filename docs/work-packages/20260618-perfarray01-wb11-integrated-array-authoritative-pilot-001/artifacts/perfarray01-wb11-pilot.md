# PERFARRAY01 WB11 Pilot

Evidence class: Static.

## Result

Stage B did not run. The package is NO-GO as scoped because the existing
production scheduler/request seam cannot host a valid WB11 array-authoritative
pilot without violating at least one of the package's two structural proofs.

## Static Trace

The current indexed scheduler is still explicitly a logical-map authority with
an indexed read mirror:

- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs:1396` documents the
  indexed path as an optional read mirror synchronized after logical writeback.
- `scheduler.rs:1568` validates consumer boundaries against
  `writeback_surface.state_surface`.
- `scheduler.rs:1606` constructs `HillslopeKernelRequest` from
  `&writeback_surface.state_surface` and `&writeback_surface.flux_surface`.
- `scheduler.rs:1676` applies accepted writeback through
  `apply_kernel_writeback` into the logical maps.
- `scheduler.rs:1714` then synchronizes the indexed mirror with
  `IndexedWritebackSurface::apply_writeback_payload`.

The kernel request type itself requires logical maps:

- `crates/openwepp-kernel-contract/src/lib_mod/core_types.rs:2453` stores
  `state_surface: &BTreeMap<BoundarySymbol, BoundaryValue>`.
- `core_types.rs:2454` stores
  `flux_surface: &BTreeMap<BoundarySymbol, BoundaryValue>`.
- `core_types.rs:2504` constructs the indexed request from logical maps plus an
  optional `IndexedWritebackSurface`.

The WB11 accessor layer still begins from logical maps for core scalar reads:

- `state_access.rs:5` / `state_access.rs:11` require state scalars via
  `request.state_surface.get`.
- `state_access.rs:28` / `state_access.rs:34` require flux scalars via
  `request.flux_surface.get`.
- `state_access.rs:151` / `state_access.rs:156` require arbitrary state symbols
  via `request.state_surface.get`.
- `state_access.rs:173` and later provide selected indexed helper paths, but
  these are not a complete replacement for the mandatory logical accessors.

The runoff reconciliation anchor immediately uses those logical accessors:

- `hydrology_phase_runoff_reconciliation.rs:10` reads rainfall input through
  `require_state_scalar`.
- `hydrology_phase_runoff_reconciliation.rs:19` reads closure tolerance through
  `require_state_scalar`.
- `hydrology_phase_runoff_reconciliation.rs:29` reads soil conductivity through
  `require_state_scalar`.
- the same function later calls snow/frost/interception/carryover helpers that
  also receive `HillslopeKernelRequest`.

## Why No Pilot Was Timed

There are only two ways to run the current WB11 runoff phase after Stage A:

1. Materialize logical `BTreeMap` state from `ArrayHotState` before invoking
   `HillslopeKernelRequest`.
2. Keep logical maps as the mutable scheduler authority and mirror writes into
   the array shell.

Option 1 violates the no per-day full `BTreeMap` export proof. Option 2
violates the no normal-path dual-write proof. A timing number from either path
would be a fictional floor under PERFARRAY01's own acceptance criteria.

## Required Rescope

The next valid package must split out the missing request/accessor/scheduler
authority boundary before timing WB11:

- add an array-capable request/view that does not require logical state/flux
  maps for hot scalar reads;
- port the WB11 scalar accessor layer to use that view for all symbols needed
  by runoff reconciliation;
- add a scheduler pilot path whose mutable authority is `ArrayHotState`;
- materialize logical maps only at explicit validation/publication boundaries.

Only after that boundary exists can a WB11 integrated floor measurement be
honest.
