# Required Reading Map

Status: `complete / WARN`

Evidence mode: `Static`

| Path | Tier | Bytes | Reason |
|---|---|---:|---|
| `AGENTS.md` | Core | 11927 | Repository invariants and routing. |
| `docs/codex_exec_plans.md` | Core | 20921 | Living ExecPlan requirements. |
| `docs/work-packages/AGENTS.md` | Core | 26013 | Package lifecycle and evidence gates. |
| `docs/work-packages/README.md` | Core | 371426 | Catalog and current package chain. |
| package-local `package.md` | Core | 12256 | Authorized objective and write set. |
| `docs/standards/testing-and-gate-strategy.md` | Core | 22200 | Direct validation lifecycle. |
| `docs/specifications/science-contract-authoring-procedure.md` | Conditional, triggered | 13715 | Canonical contract amendment procedure. |
| `docs/specifications/science-contracts/AGENTS.md` | Conditional, triggered | 5599 | Contract-local authoring rules. |
| `docs/specifications/science-contracts/kernel-process-contract-profile.md` | Conditional, triggered | 5792 | Process-contract profile. |
| `docs/specifications/science-contracts/index.md` | Conditional, triggered | 9580 | Binding contract catalog. |
| `crates/AGENTS.md` | Conditional, triggered | 5165 | Rust production edits. |
| `tests/AGENTS.md` | Conditional, triggered | 4723 | Integration-test edits. |
| `SC-SNOWFREEZE-001.md` | On-demand, triggered | Exact liquid/trace authority. |
| `SC-SNOWENERGY-001.md`, `SC-RUNOFFPART-001.md` | On-demand | Cross-boundary consistency checks only. |
| predecessor disposition and handoff | On-demand, triggered | Exact evidence gap and successor boundary. |

Core plus triggered Conditional reading totals `509317` local bytes, which is
`WARN` under the canonical `400000`/`800000` thresholds. The catalog alone is
`371426` bytes but remains Core because package-chain and active-package
reconciliation are mandatory. On-demand authority is excluded from the prompt
budget and loaded only for the touched mechanisms.
