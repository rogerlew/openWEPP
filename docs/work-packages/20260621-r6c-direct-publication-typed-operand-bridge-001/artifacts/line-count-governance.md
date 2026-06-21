# Line-Count Governance

Evidence mode: Static + Ran.

Pre-change observations:

- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
  is in the 2000-line WARN band.
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs` is in the
  2000-line WARN band.

R6C should avoid growing either file unless the work is tightly scoped and
recorded. If either file crosses 3000 lines, the package must stop or amend
scope with a split plan before closure.

Post-change counts:

- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`:
  `2896` lines, still below the 3000-line block threshold.
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`: `2609`
  lines, unchanged and below the 3000-line block threshold.

Disposition: WARN retained; no new block.
