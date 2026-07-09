# Implementation

Evidence label: Static/Ran.

Status: `ATTEMPTED-ROLLED-BACK`

Provisional implementation was attempted in
`crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/routing/02_ws20_segment_routing.rs`.

Changes:

- Added module-local characterization tests for WS20 flow partitioning, class
  transport preparation, segment hydraulics, transport snapshots, case12 update
  branches, transition eligibility, no-transition diagnostics, and no-segment
  core identity.
- Extracted behavior-preserving private helpers from `ws20_route_case34_segment`
  for dcap invocation, upper-width/depth writeback, detachment flux vector
  construction, and case3 classification.
- Extracted behavior-preserving private helpers from `ws20_route_case4_segment`
  for case4 potential loads, capacity-limited count, iterative-detach handling,
  and enddet finalization.
- Extracted `ws20_case12_transition_xdemax` from
  `ws20_try_case12_transition` to isolate the transition eligibility guard.

Preservation statement for the provisional diff:

- Extraction preserves call order, operand order, expression grouping, loop
  accumulation order, typed error behavior, and state writeback order.
- No science formulas, constants, thresholds, contract authority, guard
  semantics, serialization, diagnostics meaning, or public output semantics were
  changed.

Rollback:

- Accepted review findings blocked package completion because ADR-0021
  science-tier coverage closure was not met and key refactored case34/case4
  paths remained uncovered.
- The target Rust file was restored to the scaffold state for local hold
  closure.
- Current target line count after rollback: `1078`.
