# Required Reading Map

Status: frozen at intake

Required-reading budget: `417171` local bytes across the initial authority and
implementation set; `WARN` (`>400000`, `<=800000`). The large files are split
between core and on-demand reading; only the relevant `SC-SED-001` and solver
sections are loaded during each mechanism step.

| Tier | Path | Reason / trigger |
|---|---|---|
| Core | `AGENTS.md` | Repository invariants. |
| Core | `docs/codex_exec_plans.md` | Living ExecPlan requirements. |
| Core | `docs/defect_closure_execplans.md` | Autonomous defect-closure rules. |
| Core | `docs/work-packages/AGENTS.md` | Package lifecycle and evidence gates. |
| Core | `docs/standards/testing-and-gate-strategy.md` | Critical validation selection. |
| Core | `docs/standards/kernel-work-package-preparation.md` | Kernel intake and reading-budget rules. |
| Core | `docs/specifications/science-contracts/AGENTS.md` | Contract-first authority rules. |
| Core | `docs/planning/snow-surface-energy-balance-roadmap.md` | Campaign order and W2C gate. |
| Core | EB-04W2B `package.md` and terminal artifacts | Defect evidence and protected snow correction. |
| Conditional | `docs/specifications/science-contract-authoring-procedure.md` | Read before any SC amendment. |
| Conditional | `docs/specifications/science-contracts/kernel-process-contract-profile.md` | Read before production kernel edit. |
| Conditional | `docs/standards/prompt-wording-guidance.md` | Kickoff prompt authoring. |
| On-demand | `docs/specifications/science-contracts/contracts/SC-SED-001.md` | Relevant Wave-1 invariants, tolerances, provenance, and change log. |
| On-demand | `crates/openwepp-hillslope-orchestrator/src/direct_runtime/erosion_continuity.rs` | Numeric mechanism under diagnosis. |
| On-demand | `tests/integration/erod16_wave1_continuity_fixture_conservation.rs` | Failing real-fixture instrument and acceptance predicate. |
| On-demand | pinned `route.for`, `erod.for`, `runge.for`, and related routines | Read only for touched numeric mechanics. |

Applicable instruction chain was resolved with `tools/agents/find-agents` for
the declared documentation, crate, and test paths before edits.

