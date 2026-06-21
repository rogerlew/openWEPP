# Implementation Test Evidence

Evidence mode: Static + Ran.

## Implementation

Changed files:

- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
- `crates/openwepp-runner/src/hillslope/03_tests.rs`
- `crates/openwepp-runner/tests/r6_direct_publication_cutover_cli_contract.rs`

Static:

- `HillslopeClimateExecution` and `ClimateExecutionAccumulator` now carry
  `retained_direct_publication: Option<DirectRunPublicationFrame>`.
- `execute_hillslope_climate_days` accepts `HillslopeRuntimeSelection` and
  constructs the retained frame only for
  `DirectPublicationFrameCutover`.
- `retain_direct_publication_day_rows` appends retained rows during the climate
  day loop from:
  - `ClimateDayProjection` calendar fields;
  - `ClimateDayProjection.precipitation_mm`;
  - `per_ofe_lane_areas_m2`;
  - `output_hillslope_id` / lane index identity.
- The retained row path validates finite nonnegative precipitation, finite
  positive lane area, simulation-day index range, and lane-id range.
- The cutover branch in `build_direct_publication_artifacts` clones and
  validates `execution.retained_direct_publication`; it does not call
  `DirectRunFrame::skeleton`, `DirectFrameExecutor::new`, or
  `run_publication_capture`.
- `DirectPublicationFrameShadow` intentionally remains on the existing
  publication-capture path; R6D only lifts the cutover producer-retention
  blocker.
- `require_direct_publication_cutover_gates` now fails closed at
  `HOLD-R6D-PARITY-GRADE-PUBLICATION-PRODUCERS-ABSENT` before byte-identity
  comparison when retained rows contain only climate/calendar/geometry plus
  zero/absent publication-grade hydrology/erosion operands.

## Focused Tests

Ran:

- `cargo fmt --check` -> PASS.
- `cargo test -p openwepp-runner r6d_cutover_candidate_fails_closed_after_retained_direct_publication -- --nocapture` -> PASS.
- `cargo test -p openwepp-runner r6_direct_publication_cutover_cli_flag_fails_closed_before_outputs --test r6_direct_publication_cutover_cli_contract -- --nocapture` -> PASS.
- `cargo test -p openwepp-runner r6b_absent_operand_detector_suppresses_marker_for_nonzero_direct_operands -- --nocapture` -> PASS.
- `cargo clippy --workspace --all-targets -- -D warnings` -> PASS after extracting the climate execution completion helper.
- `cargo test --workspace` -> PASS.
- `cargo deny check` -> PASS.
- `wctl doc-lint --path docs/work-packages` -> PASS.
- `git diff --check` -> PASS.

Behavior proven:

- Cutover no longer fails at the R6C producer-retention absence marker.
- Cutover now consumes the retained frame and fails at the R6D parity-grade
  producer marker.
- Fail-closed cutover still writes no HBP, loss, WAT, PASS, or manifest files.
- Direct runtime audit counters remain zero for cutover failure:
  run-frame construction, executor construction, skeleton runs, publication
  capture, and compatibility-edge invocations are all `0`.
- Climate-only retained rows are not treated as all-zero direct rows, but they
  are still rejected as lacking parity-grade output producers.
