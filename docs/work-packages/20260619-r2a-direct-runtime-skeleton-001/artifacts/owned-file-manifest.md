# R2A Owned File Manifest

Status: complete.
Evidence mode: Static.

## Package-Owned Files

- `docs/work-packages/20260619-r2a-direct-runtime-skeleton-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`

## Conditional Rust/Test Write Set

- `crates/openwepp-hillslope-orchestrator/src/lib.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/**`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/mod.rs`
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs`
- `crates/openwepp-hillslope-orchestrator/src/phase.rs`
- `crates/openwepp-runner/src/api.rs`
- `crates/openwepp-runner/src/hillslope/**`
- `crates/openwepp-runner/src/bin/openwepp-cli-hill.rs`
- `tests/integration/**`

Execution must record every touched file, line-count status, and why it is in
scope before final disposition.

## Final Touched Files

Static:

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`: new
  direct-runtime namespace, direct-frame shells, executor skeleton, audit
  counters, and typed direct runtime errors.
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`: public re-export for
  the R2A direct runtime API.
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/mod.rs` and
  `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs`:
  focused direct-runtime tests and source-token prohibition test.
- `crates/openwepp-runner/src/api.rs`: explicit
  `HillslopeRuntimeSelection` enum.
- `crates/openwepp-runner/src/lib.rs`: public export for
  `execute_hillslope_run_with_runtime_selection`.
- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`:
  one-time direct skeleton setup selection behind explicit non-default
  runtime selection.
- `crates/openwepp-runner/src/hillslope/03_tests.rs`: default-disabled and
  opt-in runner audit tests.
- `crates/openwepp-runner/src/bin/openwepp-cli-hill.rs`: explicit
  `--direct-runtime-skeleton` opt-in flag.
- `docs/work-packages/20260619-r2a-direct-runtime-skeleton-001/**`: package
  evidence, review, verification, disposition, and handoff.
- `docs/work-packages/README.md`: execution-log update.
- `docs/ROADMAP.md`: queue update after R2A closure.

No file outside the intended or package-authorized conditional write set was
edited. No edits were made to `scheduler.rs`, phase math, publication output
code, or root integration tests.
