# R2A Line-Count Governance

Status: complete.
Evidence mode: Ran.

Before closure, run and record `.rs` line counts for touched Rust files.

Rules:

- files at or above 2000 lines are `WARN`;
- files at or above 3000 lines require refactor before closure unless an
  explicit exception with owner and sunset is recorded;
- touching `scheduler.rs` requires a strong justification and line-count
  closure plan because it is historically over 3000 lines.

## Touched Rust File Counts

Ran:

```text
   541 crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs
    78 crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs
    88 crates/openwepp-hillslope-orchestrator/src/lib.rs
    57 crates/openwepp-runner/src/api.rs
   118 crates/openwepp-runner/src/bin/openwepp-cli-hill.rs
  2487 crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs
   620 crates/openwepp-runner/src/hillslope/03_tests.rs
    32 crates/openwepp-runner/src/lib.rs
    62 crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/mod.rs
  3177 crates/openwepp-hillslope-orchestrator/src/scheduler.rs
```

Disposition:

- All newly created or directly modified Rust files are below 2000 lines except
  `00_runner_intake_and_lane_setup.rs`.
- `00_runner_intake_and_lane_setup.rs` is in WARN band at 2487 lines. The edit
  is a narrow one-time setup selector near the existing runner entrypoint.
  A split is deferred because R2A's correctness risk is the direct-runtime
  boundary, and moving the runner monolith during this package would broaden
  the write set without reducing the R2A compatibility proof risk.
- `scheduler.rs` remains above 3000 lines but was not touched. It is listed
  only as an ambient package risk check; no new exception is required for this
  package because the file has no diff.
