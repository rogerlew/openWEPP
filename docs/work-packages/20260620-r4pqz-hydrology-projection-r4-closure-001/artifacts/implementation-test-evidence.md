# Implementation Test Evidence

Status: passed.

Static: R4P/Q/Z implementation adds
`direct_runtime/projection.rs`, wires the projection span into
`DirectDayFrame::run_day_spans()`, exports the projection types/constants, and
adds focused projection fixtures in
`tests/tests_mod/direct_runtime_r4pqz.rs`.

Implemented behavior:

- typed `DirectHydrologyProjectionInputs`,
  `DirectHydrologyProjectionState`,
  `DirectHydrologyProjectionDownstreamOperands`, and
  `DirectHydrologyProjectionShadowProjection`;
- direct aggregate-storage recompute from the final R4N
  `layer_state_after_root_uptake`;
- direct state mutation, downstream operands, and shadow projection;
- required upstream shadows for R4A, R4B, R4G, R4J, R4M, R4O, and R4N;
- typed fail-closed errors for missing upstreams, invalid projection domains,
  nonfinite layer storage, and aggregate-storage mismatch;
- shadow-only publication comparison fields with `public_output_cutover =
  false`.

Ran:

- `cargo check -p openwepp-hillslope-orchestrator` passed.
- `cargo test -p openwepp-hillslope-orchestrator r4pqz -- --nocapture`
  passed: 5 tests.
- `cargo test -p openwepp-hillslope-orchestrator direct_runtime --
  --nocapture` passed: 41 tests.
- `cargo test -p openwepp-hillslope-orchestrator r4 -- --nocapture` passed:
  31 tests.
- `cargo test -p openwepp-runner r2a_ -- --nocapture` passed: 2 tests.

Focused fixture coverage:

- phase-span identity:
  `StorageReconciliation -> ClosureDiagnostics`;
- aggregate storage recompute from final direct layer state;
- direct projection state mutation, downstream operand production, and shadow
  projection;
- publication anti-aliasing and shadow-only cutover guard;
- MOFE-style lane/day transfer and carry projection identity;
- missing-upstream failure and aggregate-storage tolerance failure;
- invalid profile and nonfinite layer-domain failure.
