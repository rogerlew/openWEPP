# Required Reading Map

Status: `EXECUTED-PASS`

Evidence mode: `Static + Ran` instruction discovery.

| Path | Tier | Rationale | Trigger | Applicable instructions |
|---|---|---|---|---|
| `AGENTS.md` | Core | Repository governance | Always | `AGENTS.md` |
| `docs/codex_exec_plans.md` | Core | ExecPlan requirements | Always | `AGENTS.md` |
| `docs/work-packages/AGENTS.md` | Core | Package governance | Always | root + work-package |
| `docs/work-packages/README.md` | Core | Queue/catalog context | Always | root + work-package |
| package-local `package.md` | Core | Scope, envelope, gates | Always | root + work-package |
| `docs/defect_closure_execplans.md` | Conditional | DC conversion/HOLD rules | Before execution | root |
| science-contract authoring/profile docs | Conditional | Kernel authority conformance | Before kernel/test edits | root + science-contract |
| `crates/AGENTS.md`, `tests/AGENTS.md` | Conditional | Rust/test rules | Before Rust/test edits | root + nearest |
| `SC-ROUTE-001.md` v53 | On-demand | Canonical mechanism authority | Before vector/mechanism work | root + science-contract |

Required-reading bytes: `411669` (Core + Conditional files named in the kickoff
prompt, measured 2026-07-10 with `wc -c`).

Threshold disposition: `WARN` (`>400000`, `<=800000`). The package catalog is
the dominant file and remains Core under mandatory package-preparation
governance; canonical contracts and mechanism evidence stay On-demand.

Scaffold instruction-chain discovery was run for the declared Rust, contract,
test, queue, and package paths: root `AGENTS.md`; `crates/AGENTS.md` for Rust;
`docs/specifications/science-contracts/AGENTS.md` for the canonical contract;
`docs/work-packages/AGENTS.md` for package/catalog files.

Execution read the complete applicable instruction files before writes, the
W11B package and artifact contract, W11A handoff/dispositions, canonical
`SC-ROUTE-001` v53 mechanism sections, and the pinned `dcap.for`, `chnrt.for`,
and `wshchr.for` source lineages needed for this envelope. On-demand reading
was limited to the named channel sediment, wave routing, geometry, conservation,
and publication surfaces; no unrelated science-contract domains were loaded.
