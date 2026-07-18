# Required Reading Map

Static:

| Surface | Authority | Use |
| --- | --- | --- |
| Repository | `AGENTS.md` | Package authorization, validation, truthfulness |
| Work packages | `docs/work-packages/AGENTS.md`, `docs/codex_exec_plans.md` | Scaffold, review, closure, evidence |
| Gate decision | `docs/decisions/0039-campaign-scoped-risk-based-testing-and-assurance-gates.md` | Adopted lifecycle and transition boundaries |
| Planning | `docs/standards/testing-and-gate-strategy.md` §8 | Mechanical inputs and deterministic plan output |
| Campaign ledger | Same standard §11 | Exact head and immutable event ancestry |
| Assurance impact | Same standard §13 | Registry, watches, folds, currency, human boundary |
| Assurance lifecycle | `docs/governance/scientific-assurance-dossier-lifecycle.md` §Dependency Impact And Currency | Impact owners and transfer limits |
| Gate implementation | `crates/AGENTS.md`, adjacent planner/policy/ledger source | Typed Rust implementation |
| Contract tests | `tests/AGENTS.md`, adjacent TESTGATE tests | Deterministic behavioral proof |

Selected core authority is section-scoped where the canonical standard is
large. On-demand report manifests are inspected only for registered dependency
paths; manuscript prose and retained result contents are not implementation
inputs.
