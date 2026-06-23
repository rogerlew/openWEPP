# Line-Count Governance

Status: warn.

## Ran

- Ran: `wc -l crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs crates/openwepp-runner/src/hillslope/03_tests.rs crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers.rs crates/openwepp-runner/src/api.rs crates/openwepp-runner/src/bin/openwepp-cli-hill.rs docs/work-packages/20260623-r7e-r7h-direct-runtime-completion-001/package.md`

## Disposition

- `crates/openwepp-runner/src/hillslope/03_tests.rs`: 2442 lines, WARN
  threshold exceeded, below 3000-line closure block.
- `crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers.rs`:
  2426 lines, WARN threshold exceeded, below 3000-line closure block. This is
  the R7F blocker surface and should be split while replacing the compatibility
  day-input builder.
- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`:
  1765 lines, below WARN.
- `crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs`:
  1786 lines, below WARN.
- No touched Rust file exceeds the 3000-line closure-block threshold.
