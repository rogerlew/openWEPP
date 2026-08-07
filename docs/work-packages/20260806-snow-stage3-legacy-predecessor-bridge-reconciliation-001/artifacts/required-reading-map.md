# Required Reading Map

Status: `frozen at scaffold`.

Evidence class: `Static`.

| Path | Tier | Trigger / rationale |
| --- | --- | --- |
| `AGENTS.md` | Core | Repository invariants. |
| `docs/codex_exec_plans.md` | Core | Living ExecPlan contract. |
| `docs/work-packages/AGENTS.md` | Core | Package gates/review/closure. |
| `docs/work-packages/README.md` | Core | Canonical package routing. |
| package-local `package.md` | Core | Frozen scope and protocol. |
| `docs/specifications/science-contract-authoring-procedure.md` | Conditional | Required before contract edit. |
| `docs/specifications/science-contracts/kernel-process-contract-profile.md` | Conditional | Required before contract edit. |
| `docs/specifications/science-contracts/index.md` | Conditional | Required before contract edit. |
| `crates/AGENTS.md`, `tests/AGENTS.md` | Conditional | Required before Rust/test edit. |
| `SC-SNOWFREEZE-001.md` | On-demand | Read completely before authority or kernel edit. |
| predecessor worker handoff and disposition | On-demand | Intake and claim limits. |
| retained v3 protocol/result and historical trace | On-demand | Execution/reconstruction phases. |

`tools/agents/find-agents --for` found only root plus work-package,
science-contract, crate, and test playbooks for the declared paths.
