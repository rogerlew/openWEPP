# R2A Required Reading Map

Status: complete.
Evidence mode: Static.

Populated before Rust edits.

Required-reading bytes, excluding the new package file:

```text
212304 total
```

Disposition: `OK` (`<=400000` bytes).

| Path | Tier | Trigger | Rationale |
|---|---|---|---|
| `AGENTS.md` | Core | Always | Root governance. |
| `docs/codex_exec_plans.md` | Core | Always | ExecPlan execution and review requirements. |
| `docs/work-packages/AGENTS.md` | Core | Always | Work-package governance, subagent authorization, line counts. |
| `docs/work-packages/README.md` | Core | Always | Execution log and package discovery. |
| `docs/ROADMAP.md` | Core | Always | R2+ queue state. |
| package-local `package.md` | Core | Always | R2A scope, gates, and write set. |
| `docs/architecture/array-native-runtime-specification.md` | Core | Always | Direct-runtime architecture authority. |
| `docs/decisions/0025-array-native-hillslope-day-frame.md` | Core | Always | Existing frame decision context. |
| R0/R1 package and artifacts | Core | Always | Schema, direct type boundary, constructor/projection, no-compatibility proof plan. |
| PERFDEEP09 disposition and gates | Core | Always | R2 unblock and default-disabled regression guard. |
| `crates/AGENTS.md` | Required before Rust edits | Rust crate governance. |
| `docs/specifications/science-contracts/AGENTS.md` | Required before Rust edits | Kernel/science authority guardrails. |
| `tests/AGENTS.md` | Conditional | Before root test edits. |
| touched source files | On demand | Read before editing direct runtime, runner, or tests. |

## Execution Evidence

Static:

- Root `AGENTS.md` was provided in the execution prompt and governs this work.
- `docs/work-packages/AGENTS.md` was read before package edits.
- `crates/AGENTS.md` and
  `docs/specifications/science-contracts/AGENTS.md` were read before Rust
  edits.
- The array-native architecture specification, R0/R1 planning artifacts, and
  PERFDEEP09 disposition/gate artifacts were read before Rust edits.
- Source inventory was read for:
  `crates/openwepp-hillslope-orchestrator/src/lib.rs`,
  `crates/openwepp-hillslope-orchestrator/src/day_frame.rs`,
  `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`,
  `crates/openwepp-runner/src/api.rs`,
  `crates/openwepp-runner/src/hillslope/00_runner_intake_and_lane_setup.rs`,
  `crates/openwepp-runner/src/hillslope/01_scheduler_and_trace.rs`,
  `crates/openwepp-runner/src/hillslope/03_tests.rs`,
  `crates/openwepp-runner/src/bin/openwepp-cli-hill.rs`, and orchestrator
  test modules.
