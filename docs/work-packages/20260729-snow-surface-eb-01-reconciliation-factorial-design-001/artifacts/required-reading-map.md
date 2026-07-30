# Required Reading Map

Status: `executed`; Core total `426376` bytes (`WARN`) at execution intake.

| Path | Tier | Rationale | Trigger |
| --- | --- | --- | --- |
| `/home/workdir/openWEPP/AGENTS.md` | Core | Repository invariants | Always |
| `docs/codex_exec_plans.md` | Core | Living ExecPlan requirements | Always |
| `docs/work-packages/AGENTS.md` | Core | Package governance | Always |
| `docs/work-packages/README.md` | Core | Current catalog and retained history | Always |
| Package-local `package.md` | Core | Autonomous execution specification | Always |
| `docs/standards/testing-and-gate-strategy.md` | Conditional | Direct validation intent and terminal reconciliation | Before validation intent |
| `docs/specifications/science-contracts/AGENTS.md` | Conditional | Kernel/science authority governance | Before any proposed contract/kernel edit |
| Science-contract authoring procedure/profile/index | Conditional | Canonical authority-edit discipline | Before any proposed contract/kernel edit |
| `docs/planning/snow-surface-energy-balance-roadmap.md` | On-demand | Campaign sequence and factorial contract | Design reconciliation |
| `docs/planning/snow-frost-fidelity-strategy.md` | On-demand | Prior hypotheses, fixtures, and guardrails | Mechanism/fixture reconciliation |
| `docs/planning/paradigm2-multilayer-snow-specification.md` | On-demand | Multilayer surface-energy design | Stage 3 reconciliation |
| `SC-SNOWFREEZE-001.md` | On-demand | Canonical snow/frost authority | Equation/obligation reconciliation |
| Stage 0, Stage 3, Stage A/B, canopy-stratum, and cross-SNOTEL packages | On-demand | Retained implementation and result evidence | Relevant phase only |
| Relevant Rust source, tests, tools, and fixtures | On-demand | Current implementation truth | Named during source trace |

The core set exceeded 400,000 bytes because the work-package catalog was
`347974` bytes. The large snow contract and snow strategy remained on demand
and were read by relevant section during reconciliation. The `WARN` is accepted:
catalog authority could not truthfully be narrowed, while loading the complete
snow authorities up front would have defeated progressive disclosure.
