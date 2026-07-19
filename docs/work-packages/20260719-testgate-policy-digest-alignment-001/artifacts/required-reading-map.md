# Required Reading Map

## Core

| Path | Tier | Rationale |
| --- | --- | --- |
| `AGENTS.md` | Core | Repository invariants. |
| `docs/work-packages/AGENTS.md` | Core | Package and closure governance. |
| `docs/defect_closure_execplans.md` | Core | Defect conversion and HOLD rules. |
| `docs/standards/testing-and-gate-strategy.md` | Core | Canonical bytes bound by the policy digest. |
| `package.md` | Core | Correction envelope and gates. |
| `prompts/active/execute.md` | Core | Execution constraints. |

Core byte budget: 147,125 bytes, `OK` (`<=400000`).

## Conditional And On-Demand

| Path | Trigger |
| --- | --- |
| `tests/AGENTS.md` | Before interpreting focused test evidence. |
| `gate-policy/v1/impact-map.json` | Digest reconstruction and correction. |
| `crates/openwepp-gate-planner/src/policy.rs` | A focused planner failure requires source interpretation. |
| `tests/integration/testgate_*.rs` | A focused integration failure requires contract interpretation. |

Kernel/science authority is not applicable.
