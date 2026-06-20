# R3A Line-Count Governance

Status: complete.
Evidence mode: Static + Ran.

Ran:

```text
wc -l crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs crates/openwepp-hillslope-orchestrator/src/lib.rs crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs crates/openwepp-runner/src/hillslope/03_tests.rs
```

Result:

```text
   902 crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs
    91 crates/openwepp-hillslope-orchestrator/src/lib.rs
   244 crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs
  2488 crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs
   635 crates/openwepp-runner/src/hillslope/03_tests.rs
  4360 total
```

Disposition:

- `00_runner_intake_and_lane_setup.rs` is above the 2000-line WARN threshold,
  but the R3A edit is a single scoped production counter call on the explicit
  opt-in path. Refactoring this established runner setup file is deferred to a
  dedicated package to avoid expanding R3A scope and invalidating benchmark
  evidence.
- No newly created or direct-runtime Rust file is at or above the 2000-line
  WARN threshold.
- No touched Rust file is at or above the 3000-line required-refactor
  threshold.
- `scheduler.rs` was not touched; no scheduler line-count exception is needed.
