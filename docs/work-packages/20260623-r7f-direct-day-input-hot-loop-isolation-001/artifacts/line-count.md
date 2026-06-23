# Line-Count Governance

Status: complete.

## Policy

- `.rs` files over 2000 lines: `WARN`.
- `.rs` files over 3000 lines: closure-blocking unless exempted by local
  governance.

## Evidence

Ran:

```text
wc -l crates/openwepp-hillslope-orchestrator/src/runtime_inputs/03_climate.rs crates/openwepp-runner/src/bin/openwepp-cli-hill.rs crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs crates/openwepp-runner/src/hillslope/03_tests.rs crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers.rs crates/openwepp-runner/src/hillslope/intake_lane_setup/mod.rs
```

Result:

```text
   418 crates/openwepp-hillslope-orchestrator/src/runtime_inputs/03_climate.rs
   185 crates/openwepp-runner/src/bin/openwepp-cli-hill.rs
  1769 crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs
  2478 crates/openwepp-runner/src/hillslope/03_tests.rs
  1784 crates/openwepp-runner/src/hillslope/05_runner_execution_and_outputs.rs
  3392 crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers.rs
    26 crates/openwepp-runner/src/hillslope/intake_lane_setup/mod.rs
 10052 total
```

Disposition:

- `03_tests.rs` is over the WARN threshold but is test-local.
- `direct_publication/day_input_and_helpers.rs` remains over the
  closure-blocking threshold and is an inherited concentrated helper module.
  R7F did not split it because the package objective was hot-loop isolation,
  not a mechanical module extraction. The package added explicit source scans
  and clippy coverage to constrain the new code. A follow-up mechanical split
  remains advisable.
