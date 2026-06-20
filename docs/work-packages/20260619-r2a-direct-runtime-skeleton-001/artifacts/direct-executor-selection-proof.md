# R2A Direct Executor Selection Proof

Status: complete.
Evidence mode: Static + Ran.

Execution must prove:

- direct skeleton selection occurs once at setup;
- default compatibility execution does not construct direct skeleton state;
- opt-in/test direct skeleton selection is explicit and fail-closed;
- direct skeleton execution does not enter compatibility scheduler/kernel
  request paths;
- no per-phase compatibility branch is added to hot loops.

## Static Proof

Static:

- Default public runner entrypoint
  `execute_hillslope_run` calls
  `execute_hillslope_run_with_runtime_selection(...,
  HillslopeRuntimeSelection::Compatibility)` in
  `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`.
- `select_direct_runtime_skeleton_once` returns immediately for
  `HillslopeRuntimeSelection::Compatibility`, before constructing
  `DirectRunIdentity`, `DirectRunFrame`, or `DirectFrameExecutor`.
- The explicit opt-in path constructs the direct skeleton once after parsed
  inputs and output targets are resolved and before sidecar/static
  compatibility setup.
- `openwepp-cli-hill` defaults to `HillslopeRuntimeSelection::Compatibility`;
  `--direct-runtime-skeleton` explicitly selects
  `DirectSkeletonNoop`.
- No scheduler hot-loop branch was added;
  `git diff -- crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
  produced no diff.

## Runtime Counter Proof

Ran:

- `cargo test -p openwepp-runner r2a_ -- --nocapture` passed: 2 tests.
- `r2a_default_fixture_run_constructs_no_direct_runtime_skeleton` completed a
  compatibility fixture run and asserted all direct audit counters were zero:
  run-frame constructions, day-frame constructions, executor constructions,
  skeleton runs, phase-view constructions, forbidden compatibility calls, and
  compatibility-surface constructions.
- `r2a_explicit_direct_skeleton_selection_runs_before_compatibility_outputs`
  completed the explicit opt-in fixture path and asserted one run-frame
  construction, one executor construction, one skeleton run, nonzero day/phase
  construction.

The proof establishes that the default-disabled path constructs no R2A direct
skeleton state and that opt-in skeleton execution is explicit and fail-closed.

Review correction: direct-runtime counters now cover only direct skeleton
construction/execution. Forbidden compatibility entry absence is proven by the
direct runtime source scan and scheduler no-diff proof, avoiding misleading
always-zero compatibility counters.
