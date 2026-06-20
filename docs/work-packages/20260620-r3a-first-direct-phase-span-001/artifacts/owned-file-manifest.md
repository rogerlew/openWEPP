# R3A Owned File Manifest

Status: complete.
Evidence mode: Static + Ran.

Record every touched file and why it is in scope.

Package-owned files:

- `docs/work-packages/20260620-r3a-first-direct-phase-span-001/**`
- `docs/work-packages/README.md`
- `docs/ROADMAP.md`

Conditional Rust/test write set is defined in `package.md`.

Before closure, include `.rs` line counts and a disposition for every touched
WARN-band or over-limit file.

## Final Execution Manifest

Static:

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs`: in scope for
  selected R3A phase-span types, direct compute, mutation, downstream operands,
  shadow projection, counters, validation helpers, and typed direct runtime
  errors.
- `crates/openwepp-hillslope-orchestrator/src/lib.rs`: public exports for new
  direct runtime types/constants and the production compatibility handoff
  counter.
- `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs`:
  focused direct-span identity, invalid input rejection, no-compatibility, and
  non-tautological counter tests.
- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`:
  in scope for the explicit opt-in direct-runtime handoff counter after the
  direct skeleton returns to compatibility publication.
- `crates/openwepp-runner/src/hillslope/03_tests.rs`: in scope for
  default-disabled and explicit opt-in counter assertions.
- `docs/work-packages/20260620-r3a-first-direct-phase-span-001/**`,
  `docs/work-packages/README.md`, and `docs/ROADMAP.md`: in scope for package
  evidence and queue/catalog state.

No edits were made to `scheduler.rs`, output writers, publication schema, or
canonical `SC-*` contracts.

## Line Counts

Ran:

```text
   902 crates/openwepp-hillslope-orchestrator/src/direct_runtime.rs
    91 crates/openwepp-hillslope-orchestrator/src/lib.rs
   244 crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/direct_runtime.rs
  2488 crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs
   635 crates/openwepp-runner/src/hillslope/03_tests.rs
  4360 total
```

`00_runner_intake_and_lane_setup.rs` was already above the 2000-line WARN
threshold before this package and received a narrowly scoped handoff-counter
edit. No broad refactor is included in R3A because it would exceed the package
write set and would perturb the benchmark evidence.
