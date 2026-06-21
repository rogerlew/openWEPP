# Required Reading Map

Status: executed-hold.
Evidence mode: Static + Ran.

## Core Reading

| Document | Purpose |
|---|---|
| `AGENTS.md` | Root repository governance. |
| `docs/work-packages/AGENTS.md` | Work-package evidence, gate, review, and publication-acceptance rules. |
| `docs/specifications/science-contracts/AGENTS.md` | Contract-first and science-authority rules for publication authority. |
| `docs/codex_exec_plans.md` | ExecPlan expectations for autonomous execution. |
| `docs/ROADMAP.md` | Forward-only queue authority. |
| `docs/work-packages/README.md` | Execution log and scaffolded package catalog. |
| `docs/architecture/array-native-runtime-specification.md` | Binding array-native runtime and R6 publication architecture. |
| `docs/work-packages/r5-burndown-execplan.md` | R5 prerequisite and R6 boundary. |
| `docs/work-packages/20260621-r6-direct-publication-cutover-001/package.md` | This package contract. |

## Ledger Authority Inputs

| Document | Purpose |
|---|---|
| `docs/work-packages/20260619-perfdeep06-array-native-fast-path-inventory-001/artifacts/perfdeep06-publication-operand-ledger.md` | Seed operand ledger for R6. |
| `docs/work-packages/20260619-r0-r1-array-native-schema-frame-planning-001/artifacts/publication-ledger-promotion-plan.md` | Planning-only rule that the seed ledger must be promoted before cutover. |

## Execution Prerequisites

Read the R5E package evidence after R5E closes:

- `package.md`
- `artifacts/gate-results.md`
- `artifacts/endpoint-rss-evidence.md`
- `artifacts/no-compatibility-proof-checklist.md`
- `artifacts/disposition.md`

If those files are unavailable or R5E is not complete, R6 execution must stop
at the prerequisite gate unless a reviewed waiver is recorded. R5E is now
complete at pushed commit `d8f6bbea`.

## Source Reading

Load these files on demand during execution:

- `crates/openwepp-hillslope-orchestrator/src/direct_runtime/**`
- `crates/openwepp-hillslope-orchestrator/src/scheduler.rs`
- `crates/openwepp-runner/src/hillslope/**`
- `crates/openwepp-runner/src/totalwatsed3.rs`
- `crates/openwepp-hillslope-output/src/**`
- `crates/openwepp-watershed-output/src/**`
- `crates/openwepp-legacy-bridge/src/hbp.rs`
- `tests/integration/**`
- `tests/python/test_open_wepp_runner_api.py`

## Execution Evidence

Ran:

- R5E package evidence under
  `docs/work-packages/20260621-r5e-full-ofe-day-endpoint-readiness-001/`.
- `docs/architecture/array-native-runtime-specification.md` section `5.2.1`
  after ledger promotion.
- Static scans listed in `disposition.md` for direct publication frame and
  current output-path dependencies.

Static:

- R5E prerequisite is cleared.
- Publication ledger authority is promoted.
- Output implementation is blocked by missing run-bound direct publication
  frame.

## Gate

PASS for resumed hold scope. Required reading identified the next blocker before
production output edits.
