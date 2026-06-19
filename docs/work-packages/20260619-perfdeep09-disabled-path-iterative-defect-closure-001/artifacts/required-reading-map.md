# PERFDEEP09 Required Reading Map

Status: complete.
Evidence class: Static.

Required-reading budget:

- Local byte total: `265417`.
- Threshold disposition: `OK` (`<=400000` bytes).

| Path | Tier | Trigger | Disposition |
|---|---|---|---|
| `AGENTS.md` | Core | Always | read |
| `docs/codex_exec_plans.md` | Core | Always | read |
| `docs/defect_closure_execplans.md` | Core | Always | read |
| `docs/work-packages/AGENTS.md` | Core | Always | read |
| `docs/work-packages/README.md` | Core | Always | read and updated |
| `docs/ROADMAP.md` | Core | Always | read and updated |
| `docs/work-packages/20260619-perfdeep09-disabled-path-iterative-defect-closure-001/package.md` | Core | Always | read and updated |
| PERFDEEP08 package and artifacts | Core | Prior failed candidate evidence | read |
| PERFDEEP07 package and baseline artifacts | Core | Retained `685.85 s` point | read |
| R0/R1 planning package and hold-lift artifact | Core | R2+ hold-lift criteria | read |
| `docs/architecture/array-native-runtime-specification.md` | Core | Architecture authority | read; no edit required |
| `docs/decisions/0025-array-native-hillslope-day-frame.md` | Core | HillslopeDayFrame decision | read |
| `crates/AGENTS.md` | Conditional | Before Rust edits | read |
| `docs/specifications/science-contracts/AGENTS.md` | Conditional | Kernel/runtime authority guardrails | read |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/07_decomposition_equations.rs` | On-demand | Retained mechanism | read and edited |
| `crates/openwepp-hillslope-orchestrator/src/tests/tests_mod/boundaries.rs` | On-demand | Focused regression | read and edited |
| `crates/openwepp-kernel-contract/src/lib_mod/core_types/00_symbol_registry_and_indexed_surfaces.rs` | On-demand | Rejected candidate | read; attempted edit reverted |

`tests/AGENTS.md` and `tools/owcmp/AGENTS.md` were not required because the
retained test edit stayed under crate-local tests and no owcmp files were
changed.
