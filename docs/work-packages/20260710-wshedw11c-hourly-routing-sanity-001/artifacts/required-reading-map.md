# Required Reading Map

Status: `PASS`

Evidence mode: `Static`

## Reading Budget

- local_required_bytes_total: 338262
- threshold_outcome: `OK`
- measurement_method: `wc -c` over Core files
- measured_at_utc: `2026-07-11T03:46:23Z`

## Map

| Path | Tier | Why required | Trigger | Read timing |
|---|---|---|---|---|
| `AGENTS.md` | Core | Root governance | Always | Pre-edit |
| `docs/codex_exec_plans.md` | Core | ExecPlan requirements | Always | Pre-edit |
| `docs/work-packages/AGENTS.md` | Core | Package gates and review | Always | Pre-edit |
| `docs/work-packages/README.md` | Core | Catalog/process context | Always | Pre-edit |
| package `package.md` | Core | Local scope and gates | Always | Pre-edit |
| `crates/AGENTS.md` | Conditional | Rust crate rules | Rust test edit | Pre-edit |
| `tests/AGENTS.md` | Conditional | Test rules | Rust test edit | Pre-edit |
| science-contract `AGENTS.md` | Conditional | Authority classification | Contract-based interpretation | Phase-local |
| `SC-ROUTE-001.md` | On-demand | Routing invariants | Result interpretation | Phase-local |
| W11B evidence | On-demand | Prior release comparator | Result comparison | Phase-local |

## Instruction Chain

`tools/agents/find-agents` resolves root plus `crates/AGENTS.md` for the Rust
test and root plus `docs/work-packages/AGENTS.md` for package/catalog edits.

All triggered Core and Conditional instruction files were read before Rust
edits. `SC-ROUTE-001` and W11B evidence are used only to interpret results; no
canonical contract edit is authorized.
