# Required-Reading Map

Status: `EXECUTED-HOLD`

Evidence mode: `Static` plus `Ran` instruction discovery.

| Path | Tier | Rationale | Trigger |
|---|---|---|---|
| `AGENTS.md` | Core | Root governance and kernel completion gates | Always |
| `docs/codex_exec_plans.md` | Core | Autonomous ExecPlan requirements | Always |
| `docs/work-packages/AGENTS.md` | Core | Package closure and consumer-path rules | Always |
| `docs/work-packages/README.md` | Core | Package catalog and active context | Always |
| `docs/specifications/science-contracts/AGENTS.md` | Core | Contract-first kernel/science governance | Always |
| `docs/standards/AGENTS.md` | Core | Prompt and kernel-package standards routing | Always |
| package-local `package.md` | Core | W11 scope, phases, gates, write set | Always |
| science-contract authoring procedure/profile/index | Conditional | Required before contract edits | Contract phase |
| `crates/AGENTS.md`, `tests/AGENTS.md` | Conditional | Rust/test local guidance | Before Rust/test edits |
| `SC-ROUTE-001`, `SC-SED-001`, `SC-SYSTEM-001`, `SC-INFILE-HBP-001` | On-demand | Canonical mechanism authority | Relevant contract/test phase |
| ADR-0036 and completed M-T3 package/artifacts | On-demand | Existing hourly format and declared scope limit | Intake/contract phase |
| pinned baseline channel/sediment files | On-demand | Normative migration provenance | Baseline source-map phase |

Core byte total: `352024`.

Budget disposition: `OK` under the canonical 400,000-byte threshold.

Execution instruction discovery was refreshed for the full declared contract,
Rust, test, package, roadmap, and catalog write set. Contract paths resolved to
`AGENTS.md` plus `docs/specifications/science-contracts/AGENTS.md`; Rust/test
paths resolved to `AGENTS.md` plus `crates/AGENTS.md`; package paths resolved to
`AGENTS.md` plus `docs/work-packages/AGENTS.md`.

Scaffold discovery (`Ran`, 2026-07-10): `tools/agents/find-agents --for` on the
package, prompt, intake artifact, roadmap, and work-package catalog returned
`AGENTS.md` for all paths and `docs/work-packages/AGENTS.md` for package/catalog
paths. Execution must rerun discovery after the production/test write set is
finalized.
