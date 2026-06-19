# PERFDEEP03 Ownership Refactor

Evidence class: Static + Ran.

## Summary

PERFDEEP03 replaced the PERFDEEP02 temporary scheduler mirror with an opt-in
lane-owned compact dense state. The lane state is carried in
`OfeLanePersistentState`, borrowed by scheduler execution, updated directly from
accepted hydrology writebacks, and flushed back to logical/indexed surfaces only
at the scheduler exit boundary or diagnostic/publication boundaries.

The implementation is intentionally fail-closed behind:

```text
OPENWEPP_PERFDEEP03_LANE_DENSE_STATE=1
```

Default production execution does not activate the lane dense path.

## Implemented Shape

- `HillslopeLaneDenseState` owns compact state and flux slots in
  `crates/openwepp-hillslope-orchestrator/src/day_frame.rs`.
- Hot state and flux symbol sets are derived from indexed writeback surfaces
  through `HotSymbolTables`.
- `OfeLanePersistentState` carries the optional lane dense state across daily
  lane preparation/report replacement.
- `HillslopeKernelRequest` can borrow compact dense slot views for state and
  flux reads while retaining the logical/indexed fallback surface.
- Accepted hydrology writebacks apply directly to the lane dense state, mark
  dirty symbols, and flush only dirty slots back to logical/indexed state at the
  true scheduler boundary.
- PERFDEEP02 temporary `HillslopeDayFrame` construction is bypassed when
  PERFDEEP03 is active.

## True Boundary Materialization Set

The lane dense state materializes back to the existing surfaces at:

- scheduler exit/report boundary;
- diagnostic roundtrip evidence boundary;
- output/publication paths that still consume logical/indexed surfaces;
- default disabled path, where the dense state is absent.

There is no per-phase full-frame seed/flush loop in the PERFDEEP03 opt-in path.
The current implementation still pays partial-island edge and compatibility
costs because non-migrated publication and downstream surfaces remain live.

## Static Evidence

Primary changed files:

- `crates/openwepp-hillslope-orchestrator/src/day_frame.rs`
- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
- `crates/openwepp-hillslope-orchestrator/src/scheduler/water_balance.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/state_access.rs`
- `crates/openwepp-kernel-contract/src/lib_mod/core_types/00_symbol_registry_and_indexed_surfaces.rs`
- `crates/openwepp-kernel-contract/src/lib_mod/core_types/02_boundary_values_and_kernel_requests.rs`
- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
- `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime/03_scheduler_lifecycle.rs`

Focused test proving persistent lane ownership and direct dense update:

```text
cargo test -p openwepp-hillslope-orchestrator \
  perfdeep03_ofe_sequence_uses_lane_owned_compact_dense_state -- --nocapture
```

Result: passed.

## Line-Count Governance

Ran:

```text
wc -l \
  crates/openwepp-hillslope-orchestrator/src/scheduler.rs \
  crates/openwepp-hillslope-orchestrator/src/scheduler/water_balance.rs \
  crates/openwepp-hillslope-orchestrator/src/day_frame.rs \
  crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/writeback.rs \
  crates/openwepp-kernel-contract/src/lib_mod/core_types/00_symbol_registry_and_indexed_surfaces.rs \
  crates/openwepp-kernel-contract/src/lib_mod/core_types/02_boundary_values_and_kernel_requests.rs \
  crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs \
  crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime/03_scheduler_lifecycle.rs \
  crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/state_access.rs \
  tests/integration/mofe01_per_ofe_state_contract.rs
```

Result:

```text
2862 scheduler.rs
 277 scheduler/water_balance.rs
 791 day_frame.rs
1605 writeback.rs
1147 00_symbol_registry_and_indexed_surfaces.rs
1138 02_boundary_values_and_kernel_requests.rs
2433 00_runner_intake_and_lane_setup.rs
 792 03_scheduler_lifecycle.rs
2186 state_access.rs
 442 mofe01_per_ofe_state_contract.rs
```

No touched `.rs` file is at or above the 3000-line required-refactor threshold.
WARN-band files remain advisory follow-on hygiene, not PERFDEEP03 closure
blockers.
