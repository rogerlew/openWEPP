# Required Reading Map

Static: package scaffold required reading map.

| Path | Tier | Rationale | Applicability trigger | Read status |
|---|---|---|---|---|
| `AGENTS.md` | Core | Root repository governance and CQR authorization. | Always. | Read before scaffold. |
| `docs/work-packages/AGENTS.md` | Core | Work-package and CQR nightly process. | Always. | Read before scaffold. |
| `docs/work-packages/20260709-cqr-nightly-01-kernel-contract-typed-symbol-surfaces-001/package.md` | Core | Package-local authority and exit gates. | Always. | Authored in scaffold. |
| `docs/work-packages/cqr-nightly-burndown-execplan.md` | Core | Operator shorthand, target selection, commit and hold rules. | Always. | Read before scaffold. |
| `docs/standards/mechanical-refactor-authoring-guide.md` | Core | Behavior-preserving refactor and closure gate requirements. | Always. | Read before scaffold. |
| `docs/standards/code-quality-refactor-authoring-guide.md` | Core | CQR-specific CRAP and numeric identity requirements. | Always. | Read before scaffold. |
| `docs/decisions/0021-module-coverage-closure-thresholds.md` | Core | CRAP <= 30 and coverage closure policy. | Always. | Read before scaffold. |
| `docs/standards/prompt-wording-guidance.md` | Core | Required prompt and subagent wording. | Always. | Read before scaffold. |
| `crates/AGENTS.md` | Core | Rust crate implementation rules and line-count governance. | Target under `crates/`. | Read before scaffold. |
| `docs/specifications/science-contracts/AGENTS.md` | Core | Kernel contract boundary rules. | Target is a kernel-contract module. | Read before scaffold. |
| `crates/openwepp-kernel-contract/src/lib_mod/core_types/01_typed_symbol_surfaces.rs` | On-demand | Target module implementation. | Before implementation edits. | Read before scaffold. |
| `tests/integration/arch22_typed_state_surface_contract.rs` | On-demand | Existing symbol projection oracle. | Before characterization changes. | Read before scaffold. |
| `tests/integration/erod11_alias_boundary_ownership_contract.rs` | On-demand | Existing alias/symbol projection oracle. | Before characterization changes. | Read before scaffold. |
| Relevant `SC-*` contract | Conditional | Canonical symbol meaning authority. | Only if CQR would change symbol meaning, output semantics, guards, or authority. | Not triggered in scaffold. |
| `docs/standards/local-ci-gate-selection.md` | Conditional | Focused iteration gate selection. | Only if narrowing local loop before final gates. | Not triggered in scaffold. |
