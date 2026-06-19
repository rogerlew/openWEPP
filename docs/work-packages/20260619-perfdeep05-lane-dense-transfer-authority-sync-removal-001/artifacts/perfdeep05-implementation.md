# PERFDEEP05 Implementation

Evidence class: Static + Ran.

## Summary

PERFDEEP05 removed the PERFDEEP04 full lane-dense resync call from the
PERFDEEP03 opt-in OFE daily loop and made MOFE transfer input mutate the
lane-owned dense state directly.

The path remains opt-in behind `OPENWEPP_PERFDEEP03_LANE_DENSE_STATE=1`.
Default production execution is still disabled.

## Production Changes

- Added cached transfer symbol ids in
  `crates/openwepp-hillslope-orchestrator/src/scheduler.rs` through
  `LaneDenseTransferSymbolIds`.
- Replaced the hot-loop transfer branch with
  `apply_next_transfer_input_to_lane`, which:
  - uses the existing logical transfer materialization for non-dense/default
    execution;
  - keeps logical transfer materialization for opt-in diagnostics and
    non-migrated consumers without indexed symbol lookup;
  - applies transfer scalars and hourly carry arrays directly to
    `HillslopeLaneDenseState`;
  - updates the indexed writeback surface by cached `SymbolId`.
- Added dense dirty-set setters in
  `crates/openwepp-hillslope-orchestrator/src/day_frame.rs`:
  `set_state_value_dirty` and `set_flux_value_dirty`.
- Added `refresh_cached_slots_from_writeback_surface`, a cached-slot refresh
  that iterates existing dense slot ids and does not rebuild hot symbol lists.
- Added a runner daily-preparation refresh in
  `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime/03_scheduler_lifecycle.rs`
  so carried dense state sees freshly prepared daily hot/static symbols before
  scheduler execution.
- Added a scheduler coherence bridge for non-island phase writebacks so later
  dense-island reads do not see stale dense values after an unmigrated phase
  writes a symbol.

## Focused Tests

Added focused PERFDEEP05 tests in
`crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/writeback.rs`:

- `perfdeep05_ofe_sequence_applies_transfer_input_through_lane_dense_state`
- `perfdeep05_non_island_writeback_refreshes_later_lane_dense_reads`
- `perfdeep05_cached_slot_refresh_populates_prepared_hot_static_symbols`
- `perfdeep05_scheduler_hot_loop_does_not_sync_lane_dense_from_writeback`

Ran:

```text
cargo test -p openwepp-hillslope-orchestrator perfdeep05 -- --nocapture
```

Result: passed, `4` tests.

## Correctness Note

The first H2637 opt-in endpoint attempt after removing the old full sync failed
on day 2 with `HKERNEL-WB11-DRAIN-E-001`: the dense state was missing prepared
daily hot/static symbol `wb18_perc_fc_0001`. The old full sync had been masking
that stale dense-state defect. The new cached-slot refresh in daily preparation
fixes that without reintroducing `HotSymbolTables::hot_state_symbols` rebuilds
in the scheduler hot loop.
