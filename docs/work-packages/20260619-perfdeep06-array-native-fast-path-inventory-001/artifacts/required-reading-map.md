# PERFDEEP06 Required Reading Map

Status: complete 2026-06-19.
Evidence class: Static + Ran.

## Budget

Local byte total: `209369` bytes for the scaffolded core set.
Threshold disposition: `OK` (`OK <=400000`, `WARN >400000`,
`REQUIRES-JUSTIFICATION >800000`).

Ran:

- `wc -c AGENTS.md docs/codex_exec_plans.md docs/work-packages/AGENTS.md docs/work-packages/README.md docs/ROADMAP.md docs/work-packages/20260619-perfdeep06-array-native-fast-path-inventory-001/package.md docs/architecture/array-native-runtime-specification.md docs/decisions/0025-array-native-hillslope-day-frame.md docs/work-packages/20260619-perfdeep05-lane-dense-transfer-authority-sync-removal-001/package.md docs/work-packages/20260619-perfdeep05-lane-dense-transfer-authority-sync-removal-001/artifacts/perfdeep05_disposition.md docs/work-packages/20260619-perfdeep05-lane-dense-transfer-authority-sync-removal-001/artifacts/perfdeep05-profile.md`

Static: the core set was read or re-read for this execution. Conditional files
were read where applicable: `crates/AGENTS.md` and
`docs/specifications/science-contracts/AGENTS.md` were already required by the
package setup and no Rust/contract edits were made.

## Core

| Path | Tier | Rationale | Applicability |
|---|---|---|---|
| `AGENTS.md` | Core | Root governance and validation rules. | Always before execution. |
| `docs/codex_exec_plans.md` | Core | ExecPlan and review/disposition requirements. | Always before execution. |
| `docs/work-packages/AGENTS.md` | Core | Work-package artifact, gate, and subagent requirements. | Always before execution. |
| `docs/work-packages/README.md` | Core | Package catalog and queue/execution-log conventions. | Always before execution. |
| `docs/ROADMAP.md` | Core | Canonical forward queue and PERF track state. | Always before execution. |
| `docs/work-packages/20260619-perfdeep06-array-native-fast-path-inventory-001/package.md` | Core | Package authority. | Always before execution. |
| `docs/architecture/array-native-runtime-specification.md` | Core | Binding ADR-0025 design specification and Stage-3 scope. | Always before execution. |
| `docs/decisions/0025-array-native-hillslope-day-frame.md` | Core | Ratified design authority and required gates. | Always before execution. |
| `docs/work-packages/20260619-perfdeep05-lane-dense-transfer-authority-sync-removal-001/package.md` | Core | Immediate predecessor package scope. | Always before execution. |
| `docs/work-packages/20260619-perfdeep05-lane-dense-transfer-authority-sync-removal-001/artifacts/perfdeep05_disposition.md` | Core | Immediate no-go and follow-on evidence. | Always before execution. |
| `docs/work-packages/20260619-perfdeep05-lane-dense-transfer-authority-sync-removal-001/artifacts/perfdeep05-profile.md` | Core | Measured remaining dense-edge costs. | Always before execution. |

## Conditional

| Path | Tier | Rationale | Applicability |
|---|---|---|---|
| `crates/AGENTS.md` | Conditional | Rust crate rules. | Required before Rust crate edits. |
| `docs/specifications/science-contracts/AGENTS.md` | Conditional | Kernel/science authority rules. | Required before runtime-projection edits controlling kernel branches. |
| `docs/specifications/science-contract-authoring-procedure.md` | Conditional | Contract amendment procedure. | Required if canonical contract authority must change. |
| `docs/specifications/science-contracts/kernel-process-contract-profile.md` | Conditional | Kernel package profile requirements. | Required if canonical contract authority must change. |
| `docs/specifications/science-contracts/index.md` | Conditional | Contract registry. | Required if canonical contract authority must change. |
| `tests/AGENTS.md` | Conditional | Test conventions. | Required before test edits. |

## On Demand

| Path | Tier | Rationale | Applicability |
|---|---|---|---|
| PERFDEEP03 artifacts | On-demand | Lane-owned compact dense state predecessor. | Load when tracing ownership or identity gates. |
| PERFDEEP04 artifacts | On-demand | Profile that identified the full-sync hotspot. | Load when comparing hotspot sequence. |
| `crates/openwepp-hillslope-orchestrator/src/day_frame.rs` | On-demand | Transition dense frame and lane state. | Load for working-set/API inventory. |
| `crates/openwepp-hillslope-orchestrator/src/scheduler.rs` | On-demand | Daily/OFE/phase loop and mixed-mode boundaries. | Load for working-set/API inventory. |
| `crates/openwepp-hillslope-orchestrator/src/phase.rs` | On-demand | Phase order. | Load for phase API inventory. |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/**` | On-demand | Current phase bodies and state access. | Load for direct-frame API planning. |
| `crates/openwepp-kernel-contract/src/lib_mod/core_types/00_symbol_registry_and_indexed_surfaces.rs` | On-demand | Registry/indexed surface machinery. | Load for no-hot-loop-map proof. |
| `crates/openwepp-kernel-contract/src/lib_mod/core_types/02_boundary_values_and_kernel_requests.rs` | On-demand | Kernel request/response/writeback contracts. | Load for direct-frame API planning. |
| `crates/openwepp-runner/src/hillslope/**` | On-demand | HBP/WAT/PASS publication construction. | Load for publication operand ledger. |
| `crates/openwepp-hillslope-output/src/**` | On-demand | Typed output row schemas. | Load for publication operand ledger. |
