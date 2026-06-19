# Required Reading Map

Status: complete.
Evidence mode: Static/Ran.

## Core Reading

Ran:

```text
wc -c AGENTS.md docs/codex_exec_plans.md docs/work-packages/AGENTS.md docs/work-packages/README.md docs/ROADMAP.md docs/work-packages/20260619-perfdeep08-disabled-path-hard-isolation-001/package.md docs/work-packages/20260619-perfdeep07-zero-cost-disabled-direct-frame-hydrology-001/package.md docs/work-packages/20260619-perfdeep07-zero-cost-disabled-direct-frame-hydrology-001/artifacts/disposition.md docs/work-packages/20260619-perfdeep07-zero-cost-disabled-direct-frame-hydrology-001/artifacts/gate-results.md docs/work-packages/20260619-perfdeep07-zero-cost-disabled-direct-frame-hydrology-001/artifacts/perfdeep07-disabled-path-audit.md docs/work-packages/20260619-perfdeep07-zero-cost-disabled-direct-frame-hydrology-001/artifacts/perfdeep07-zero-cost-disabled-proof.md docs/work-packages/20260619-perfdeep07-zero-cost-disabled-direct-frame-hydrology-001/artifacts/worker-handoff.md docs/work-packages/20260619-r0-r1-array-native-schema-frame-planning-001/package.md docs/work-packages/20260619-r0-r1-array-native-schema-frame-planning-001/artifacts/perfdeep07-hold-lift-disposition.md docs/work-packages/20260619-r0-r1-array-native-schema-frame-planning-001/artifacts/no-compatibility-proof-plan.md docs/architecture/array-native-runtime-specification.md docs/decisions/0025-array-native-hillslope-day-frame.md crates/AGENTS.md docs/specifications/science-contracts/AGENTS.md
```

Total core reading budget recorded: `238492` bytes.

Read before edits:

- root `AGENTS.md`;
- `docs/work-packages/AGENTS.md`;
- package spec and active kickoff prompt;
- `crates/AGENTS.md`;
- `docs/specifications/science-contracts/AGENTS.md`;
- PERFDEEP07 package, disabled audit, proof, gate results, disposition, and
  worker handoff;
- R0/R1 package, hold-lift disposition, and no-compatibility proof plan.

## Source Files Inspected

- `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`
- `crates/openwepp-runner/src/hillslope/indexed_shadow_surface.rs`
- `crates/openwepp-runner/src/hillslope/scheduler_trace/perfdeep02_frame_roundtrip.rs`
- `crates/openwepp-runner/src/hillslope/scheduler_trace/scheduler_seed_and_runtime/03_scheduler_lifecycle.rs`
- `crates/openwepp-runner/src/hillslope/scheduler_trace/hphys_trace.rs`
- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/state_access.rs`
- `crates/openwepp-kernel-contract/src/lib_mod/core_types/02_boundary_values_and_kernel_requests.rs`

## Conditional Reading

Science-contract AGENTS was read before Rust edits. No canonical contract file
was read or amended because no physics, guard, output, unit, or diagnostic
authority change was made.
