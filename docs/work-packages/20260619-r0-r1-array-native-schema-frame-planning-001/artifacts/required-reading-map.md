# Required Reading Map

Status: complete.
Evidence mode: Static/Ran.

## Budget

Ran:

```text
wc -c AGENTS.md docs/codex_exec_plans.md docs/work-packages/AGENTS.md docs/work-packages/README.md docs/architecture/array-native-runtime-specification.md docs/decisions/0025-array-native-hillslope-day-frame.md docs/work-packages/20260619-perfdeep06-array-native-fast-path-inventory-001/artifacts/perfdeep06-working-set-inventory.md docs/work-packages/20260619-perfdeep06-array-native-fast-path-inventory-001/artifacts/perfdeep06-publication-operand-ledger.md docs/work-packages/20260619-perfdeep06-array-native-fast-path-inventory-001/artifacts/perfdeep06-direct-frame-api-plan.md docs/work-packages/20260619-perfdeep07-zero-cost-disabled-direct-frame-hydrology-001/artifacts/disposition.md docs/work-packages/20260619-perfdeep07-zero-cost-disabled-direct-frame-hydrology-001/artifacts/gate-results.md
```

Total: `179056` bytes. Disposition: `OK` (`<=400000` bytes).

## Core

| Path | Bytes | Rationale |
|---|---:|---|
| `AGENTS.md` | `9043` | root package/runtime governance |
| `docs/codex_exec_plans.md` | `20443` | autonomous execution expectations |
| `docs/work-packages/AGENTS.md` | `10235` | work-package gates and artifact rules |
| `docs/work-packages/README.md` | `71621` | package catalog and queue context |
| `docs/architecture/array-native-runtime-specification.md` | `37751` | R0/R1 authority and hold-lift rules |
| `docs/decisions/0025-array-native-hillslope-day-frame.md` | `6487` | ratifying ADR |
| `docs/work-packages/20260619-perfdeep06-array-native-fast-path-inventory-001/artifacts/perfdeep06-working-set-inventory.md` | `6752` | schema and working-set seed |
| `docs/work-packages/20260619-perfdeep06-array-native-fast-path-inventory-001/artifacts/perfdeep06-publication-operand-ledger.md` | `7268` | publication ledger seed |
| `docs/work-packages/20260619-perfdeep06-array-native-fast-path-inventory-001/artifacts/perfdeep06-direct-frame-api-plan.md` | `5450` | direct-frame API seed |
| `docs/work-packages/20260619-perfdeep07-zero-cost-disabled-direct-frame-hydrology-001/artifacts/disposition.md` | `1559` | active HOLD blocker |
| `docs/work-packages/20260619-perfdeep07-zero-cost-disabled-direct-frame-hydrology-001/artifacts/gate-results.md` | `2447` | failed P0 timing evidence |

## Conditional

| Path | Trigger | Disposition |
|---|---|---|
| `docs/specifications/science-contracts/AGENTS.md` | contract, guard, unit, output-meaning, or conservation authority edits | Not triggered; package is docs-only planning. |
| `crates/AGENTS.md` | Rust crate edits | Not triggered. |
| `tests/AGENTS.md` | test edits | Not triggered. |

## On Demand

| Path | Reason Used |
|---|---|
| `crates/openwepp-hillslope-orchestrator/src/day_frame.rs` | static proof that current `HillslopeDayFrame` is compatibility-backed |
| `crates/openwepp-hillslope-orchestrator/src/scheduler.rs` | static forbidden API inventory |
| `crates/openwepp-runner/src/hillslope/02_output_and_climate_helpers.rs` | publication projection planning context |
| `crates/openwepp-kernel-contract/src/lib_mod/core_types/**` | request/payload/symbol terminology |
