# Characterization

Evidence label: Static/Ran.

Status: `COMPLETE`

Baseline observations:

- Existing baseline coverage reaches the target through current watershed
  orchestrator/runner tests, but `impoundment_outflow_at_stage` remains high
  CRAP with only `35.15151515151515%` line coverage.
- Characterization must cover WS12 outlet-family behavior and adaptive retry
  semantics before decomposition.
- Because `helpers.rs` is included before later kernel sections, any
  module-local tests must avoid a trailing `#[cfg(test)] mod ...` block that
  triggers full-clippy `items_after_test_module`.

Added characterization in
`crates/openwepp-watershed-orchestrator/src/lib_mod/kernel/helpers.rs` for:

- outlet-family aggregation across drop spillway, culvert, rockfill,
  emergency-spillway, filter-fence, and perforated-riser families;
- typed fail-closed guards for invalid culvert, rockfill, riser, total-outflow,
  area, continuity, RK4 timestep, adaptive timestep, and pool-capacity domains;
- adaptive retry semantics for regime-transition and truncation-error control;
- route-wrapper duration, multi-step, and iteration-limit behavior.

Focused command evidence:

- Test-first chronology: the initial outlet-family and adaptive-step
  characterization tests were added before production decomposition, then
  `cargo nextest run -p openwepp-watershed-orchestrator` passed with
  `28 tests run: 28 passed, 0 skipped` after fixture correction.
- Additional guard and coverage-floor tests were added during coverage closure
  after the extraction; they characterize existing private fail-closed branches
  and do not change production behavior.
- `cargo nextest run -p openwepp-watershed-orchestrator` - exit `0`,
  `39 tests run: 39 passed, 0 skipped`.
- `cargo clippy -p openwepp-watershed-orchestrator -- -D warnings` - exit `0`.
- `cargo llvm-cov -p openwepp-watershed-orchestrator --lcov --output-path /tmp/openwepp-cqr-nightly-05-helpers-focused.lcov`
  - exit `0`, `39` unit tests passed under coverage.
- `cargo llvm-cov -p openwepp-watershed-orchestrator --json --summary-only --output-path /tmp/openwepp-cqr-nightly-05-helpers-focused-summary.json`
  - exit `0`, target file region summary recorded.

Behavior oracle:

- The tests assert closed-form outflow sums for active outlet families and exact
  stable-step stage advance (`hnext = 1.1`, accepted `dt = 1.0`) for the
  constant-rate integration fixture.
- The tests assert existing typed `BoundaryClass` outcomes for invalid domains
  instead of changing guard posture or silently normalizing inputs.
- The error-control retry test asserts an accepted smaller timestep
  (`accepted_dt < 1.0`) for a deterministic linear outflow fixture, so it would
  fail if the adaptive error retry branch were skipped.
