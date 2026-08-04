# Required Reading Map

Status: `complete / WARN`

Evidence mode: `Static`

Applicable instruction discovery ran before scaffold edits:

```text
tools/agents/find-agents --for \
  docs/work-packages/20260803-snow-mass-transition-ledger-persistence-001/package.md \
  docs/work-packages/README.md \
  docs/ROADMAP.md \
  docs/planning/snow-surface-energy-balance-roadmap.md
```

The package tree and catalog inherit `AGENTS.md` then
`docs/work-packages/AGENTS.md`. Both roadmap files inherit root `AGENTS.md`.
Future contract, crate, and test edits additionally inherit the nearest
science-contract, crate, and test guidance named below.

| Path | Tier | Bytes | Reason |
|---|---|---:|---|
| `AGENTS.md` | Core | 11927 | Repository invariants and routing. |
| `docs/codex_exec_plans.md` | Core | 20921 | Living ExecPlan requirements. |
| `docs/work-packages/AGENTS.md` | Core | 26013 | Package lifecycle and evidence gates. |
| `docs/work-packages/README.md` | Core | 371975 | Catalog and current package chain. |
| package-local `package.md` | Core | 22825 | Authorized objective and write set. |
| `docs/standards/testing-and-gate-strategy.md` | Core | 22200 | Direct validation lifecycle. |
| `docs/standards/AGENTS.md` | Conditional, triggered | 3667 | Standards-local routing. |
| `docs/standards/mechanical-refactor-authoring-guide.md` | Conditional, triggered | 10843 | Behavior-neutral seam and API parity. |
| `docs/standards/local-ci-gate-selection.md` | Conditional, triggered | 5655 | Direct command selection. |
| `docs/specifications/science-contract-authoring-procedure.md` | Conditional, triggered | 13715 | Canonical contract amendment procedure. |
| `docs/specifications/science-contracts/AGENTS.md` | Conditional, triggered | 5599 | Contract-local authoring rules. |
| `docs/specifications/science-contracts/kernel-process-contract-profile.md` | Conditional, triggered | 5792 | Runtime projection and kernel branch profile. |
| `docs/specifications/science-contracts/index.md` | Conditional, triggered | 9580 | Binding contract catalog. |
| `crates/AGENTS.md` | Conditional, triggered | 5165 | Rust production edits. |
| `tests/AGENTS.md` | Conditional, triggered | 4723 | Integration-test edits. |
| `SC-SNOWFREEZE-001.md` | On-demand, triggered | excluded | Exact snow transition and trace authority. |
| `SC-SNOWENERGY-001.md`, `SC-RUNOFFPART-001.md` | On-demand | excluded | Cross-boundary consistency only. |
| predecessor audit and trace-closure artifacts | On-demand, triggered | excluded | Hold rationale, operand lineage, and proven consumer. |

Core plus triggered Conditional reading totals `540600` local bytes, which is
`WARN` under the canonical `400000`/`800000` thresholds. The catalog alone is
`371975` bytes but remains Core because package-chain and active-package
reconciliation are mandatory. On-demand files remain excluded from the prompt
budget and are loaded only when their mechanism is touched.
