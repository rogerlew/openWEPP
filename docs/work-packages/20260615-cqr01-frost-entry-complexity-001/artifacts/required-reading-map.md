# CQR01 Required Reading Map

Status: complete

## Static

| Path | Tier | Bytes | Rationale |
|---|---:|---:|---|
| `AGENTS.md` | Core | 9043 | Root repo governance and package authorization rules. |
| `docs/codex_exec_plans.md` | Core | 20443 | Work-package review, verification, and gate expectations. |
| `docs/work-packages/AGENTS.md` | Core | 10235 | Package execution, evidence, and closure rules. |
| `docs/work-packages/README.md` | Core | 28392 | Package catalog and queue context. |
| `docs/work-packages/20260615-cqr01-frost-entry-complexity-001/package.md` | Core | 7747 | Local package authority at scaffold time. |
| `docs/standards/AGENTS.md` | Conditional | 3314 | Standards maintenance and prompt guidance. |
| `docs/standards/mechanical-refactor-authoring-guide.md` | Conditional | 10569 | Behavior-preserving refactor closure loop. |
| `docs/standards/code-quality-refactor-authoring-guide.md` | Conditional | 10087 | Code-quality refactor metric and numeric-equivalence rules. |
| `docs/standards/module-test-enhancement-authoring-guide.md` | Conditional | 11190 | Coverage/test precondition for code-quality work. |
| `docs/standards/kernel-work-package-preparation.md` | Conditional | 11601 | Kernel package scaffold requirements. |
| `docs/specifications/science-contracts/AGENTS.md` | Conditional | 5585 | Contract-first guardrails for kernel-adjacent code. |
| `crates/AGENTS.md` | Conditional | 4450 | Rust crate authoring and validation rules. |
| `crates/openwepp-hillslope-orchestrator/src/hydrology/support_helpers_mod/coupling/frost_entry.rs` | On-demand | 39755 | Target source file. |

Local bytes total at scaffold time: 164664.

Budget disposition: OK (`<=400000` bytes).

## Ran

- `wc -c ...`
  - exit_code: 0
  - result: local bytes total `164664`.
