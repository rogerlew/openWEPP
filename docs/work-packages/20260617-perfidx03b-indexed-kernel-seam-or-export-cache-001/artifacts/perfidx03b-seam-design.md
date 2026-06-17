# PERFIDX03B Seam Design

Static: reviewed the current scheduler, runner, and indexed surface code.
Ran: focused compile and regression tests during implementation.

## Decision

PERFIDX03B closes the PERFIDX03 blocker with an export-cache seam rather than
widening the kernel writeback payload shape.

The persistent multi-OFE lane state now owns:

- the existing logical `HillslopeWritebackSurface` as the kernel-readable export
  cache;
- an optional `IndexedWritebackSurface` mirror activated against the frozen run
  symbol registry.

The hot runner path moves the logical export cache into scheduler execution with
`OfeLanePersistentState::take_execution_input()` instead of cloning the full
`BTreeMap` surface for every lane/day. After the sequence report is moved back
into persistent state and PL sentinels are restored, the indexed mirror is
refreshed from the current logical cache.

## Implementation Notes

- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
  - Added indexed activation/refresh methods on `OfeLanePersistentState` and
    `OfeLanePersistentStateSequence`.
  - Added `take_execution_input()` for move-based persistent execution inputs.
  - Kept the existing clone-based public scheduler method unchanged so
    fail-closed prior-state preservation remains intact.
- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
  - Builds one frozen symbol registry during static hillslope setup.
  - Activates persistent lane indexed writeback mirrors when persistent lane
    state exists.
  - Carries the registry through the day scheduler lifecycle context.
- `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime.rs`
  - Moves persistent lane surfaces into execution inputs.
  - Moves report surfaces back, restores PL sentinels, then refreshes indexed
    authority.
- `crates/openwepp-kernel-contract/src/lib_mod/core_types.rs`
  - Reworked indexed surface creation as a sorted registry/surface merge. This
    removes the redundant per-surface sort and per-entry registry lookup while
    preserving unknown-symbol fail-closed behavior.
- `crates/openwepp-runner/src/hillslope/symbol_registry_audit.rs`
  - Added conservative frost fine-layer registry coverage for valid first-day
    multi-OFE frost symbol production.

## Boundary Preservation

- External/logical seams still use `BoundarySymbol` maps.
- Kernel writeback payload shape is unchanged.
- No `SC-*` contract was changed.
- No irrigation activation or sidecar wiring was added.

