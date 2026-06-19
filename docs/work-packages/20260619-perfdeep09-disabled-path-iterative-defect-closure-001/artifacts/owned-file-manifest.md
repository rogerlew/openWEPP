# PERFDEEP09 Owned File Manifest

Status: complete.
Evidence class: Static.

## Touched Files

| File | Rationale | In package envelope |
|---|---|---|
| `crates/openwepp-hillslope-orchestrator/src/hydrology/07_decomposition_equations.rs` | Retained one-pass indexed-overflow guard for perennial decomposition control. | yes, `crates/openwepp-hillslope-orchestrator/src/hydrology/**` |
| `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/boundaries.rs` | Focused fail-closed regression for unexpected indexed perennial symbols. | yes, `crates/openwepp-hillslope-orchestrator/src/tests.rs` / crate-local tests |
| `docs/work-packages/20260619-perfdeep09-disabled-path-iterative-defect-closure-001/**` | Package execution evidence and disposition. | yes |
| `docs/work-packages/README.md` | Execution-log update. | yes |
| `docs/ROADMAP.md` | Forward-queue state update after blocker closure. | yes |

## Reverted / Not Retained

- `crates/openwepp-kernel-contract/src/lib_mod/core_types/00_symbol_registry_and_indexed_surfaces.rs`
  was temporarily changed during rejected candidate 1 and reverted before the
  retained candidate. No final diff remains.

## Protected Boundaries

- No `SC-*` contract file changed.
- No output schema, unit, metadata meaning, process-physics formula, direct
  executor, direct-frame hydrology runtime, or default opt-in activation was
  changed.
