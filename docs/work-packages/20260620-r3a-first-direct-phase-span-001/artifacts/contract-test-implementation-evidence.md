# R3A Contract-Test Implementation Evidence

Status: complete.
Evidence mode: Static + Ran.

Focused tests added or extended:

- `r3a_input_accounting_span_computes_mutates_downstream_and_shadow_projects`
  proves selected span order, typed inputs, direct compute, direct state
  mutation, downstream operands, shadow projection, and zero compatibility edge
  invocations (`direct_runtime.rs` tests).
- `r3a_input_accounting_span_rejects_invalid_inputs` proves fail-closed
  nonfinite and negative direct-frame value handling.
- `r2a_direct_skeleton_runs_noop_and_records_only_direct_audit_counters` now
  proves explicit skeleton execution runs the R3A span once per seeded direct
  lane and records direct counters.
- Runner R2A fixture tests now prove default-disabled compatibility execution
  leaves all direct span counters at zero and explicit opt-in records span
  execution counters plus one production compatibility-edge handoff.

Ran:

- `cargo test -p openwepp-hillslope-orchestrator r3a_ -- --nocapture`: PASS,
  3 tests.
- `cargo test -p openwepp-hillslope-orchestrator r2a_direct -- --nocapture`:
  PASS, 3 tests.
- `cargo test -p openwepp-runner r2a_ -- --nocapture`: PASS, 2 tests.

No test claims R4 hydrology-path, R6 publication, endpoint-improvement, or
default-activation readiness.
