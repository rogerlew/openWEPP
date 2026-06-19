# PERFDEEP05 Static Hot-Loop Proof

Evidence class: Static.

## Search

Ran after final code cleanup:

```text
rg -n "sync_from_writeback_surface|refresh_cached_slots_from_writeback_surface|apply_transfer_input_to_lane_dense_state|apply_next_transfer_input_to_lane|LaneDenseTransferSymbolIds" crates/openwepp-hillslope-orchestrator/src crates/openwepp-runner/src
```

Relevant result:

```text
crates/openwepp-hillslope-orchestrator/src/day_frame.rs:197:    pub fn sync_from_writeback_surface(
crates/openwepp-hillslope-orchestrator/src/day_frame.rs:323:    pub fn refresh_cached_slots_from_writeback_surface(
crates/openwepp-hillslope-orchestrator/src/scheduler.rs:961:struct LaneDenseTransferSymbolIds {
crates/openwepp-hillslope-orchestrator/src/scheduler.rs:2217:            apply_next_transfer_input_to_lane(
crates/openwepp-hillslope-orchestrator/src/scheduler.rs:2491:fn apply_next_transfer_input_to_lane(
crates/openwepp-hillslope-orchestrator/src/scheduler.rs:2551:    apply_transfer_input_to_lane_dense_state(
crates/openwepp-hillslope-orchestrator/src/scheduler.rs:2560:fn apply_transfer_input_to_lane_dense_state(
crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/writeback.rs:557:        !scheduler_source.contains(".sync_from_writeback_surface("),
crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime/03_scheduler_lifecycle.rs:279:                .refresh_cached_slots_from_writeback_surface(
```

## Classification

`sync_from_writeback_surface` remains as a method definition only. No production
call site remains in `scheduler.rs`, and the focused test asserts that
`.sync_from_writeback_surface(` is absent from scheduler source.

The PERFDEEP03 opt-in OFE sequence now calls
`apply_next_transfer_input_to_lane`, which caches transfer symbol ids once and
routes active lane-dense transfer input to
`apply_transfer_input_to_lane_dense_state`.

`refresh_cached_slots_from_writeback_surface` is not the removed full sync. It
does not call `HotSymbolTables::hot_state_symbols`; it iterates the dense state's
existing slot-id vectors and refreshes those cached slots from prepared
logical/indexed surfaces. It is used during runner daily preparation to keep
carried dense state coherent with daily static/runtime preparation.

## Conclusion

Acceptance criterion met: `sync_from_writeback_surface` is not called in the
PERFDEEP03 opt-in H2637 daily hot loop.
