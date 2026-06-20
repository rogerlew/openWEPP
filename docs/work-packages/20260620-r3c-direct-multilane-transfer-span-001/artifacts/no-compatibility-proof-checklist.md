# No-Compatibility Proof Checklist

Status: complete.
Evidence mode: Ran.

Required proof surfaces:

| Check | Result | Evidence |
|---|---:|---|
| Direct runtime forbidden-token source scan | PASS | `rg -n "SymbolRegistry|BoundarySymbol|BoundaryValue|Option<BoundaryValue>|HillslopeWritebackSurface|KernelWritebackPayload|IndexedWritebackSurface|HotSymbolTables|HillslopeKernelRequest|execute_with_kernel|state_value_for_symbol|flux_value_for_symbol|dirty_state_ids|dirty_flux_ids" crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs` exited `1` with no matches. |
| Scheduler diff remains empty | PASS | `git diff -- crates/openwepp-hillslope-orchestrator/src/scheduler.rs` produced no diff. |
| Default-disabled runtime counters remain zero | PASS | `cargo test -p openwepp-runner r2a_default_fixture_run_constructs_no_direct_runtime_skeleton -- --nocapture`; also covered in `cargo test -p openwepp-runner r2a_ -- --nocapture`. |
| Explicit opt-in direct counters are positive | PASS | `cargo test -p openwepp-runner r2a_explicit_direct_skeleton_selection_runs_before_compatibility_outputs -- --nocapture`; R3C assertions require `phase_span_runs >= 3`, phase entries include R3A+R3B+R3C, and direct compute/state/downstream/shadow counts are `>= 3`. |
| Direct span compatibility-edge count remains zero | PASS | Focused R3C test asserts `report.compatibility_edge_invocation_count == 0`; aggregate direct skeleton test asserts report/audit compatibility-edge counts are zero inside direct runtime. |
