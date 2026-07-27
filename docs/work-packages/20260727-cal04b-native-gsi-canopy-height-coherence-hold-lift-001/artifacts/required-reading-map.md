# Required Reading Map

Status: `ACTIVE / WARN BUDGET`

Evidence class: `Static`

| Tier | Path | Trigger / rationale |
|---|---|---|
| Core | `AGENTS.md` | Repository invariants |
| Core | `docs/codex_exec_plans.md` | ExecPlan lifecycle |
| Core | `docs/work-packages/AGENTS.md` | Package closure and review |
| Core | `docs/work-packages/README.md` | Queue/catalog state |
| Core | package-local `package.md` | Exact authority envelope |
| Conditional | `docs/defect_closure_execplans.md` | DC conversion and HOLD rules |
| Conditional | `docs/specifications/science-contract-authoring-procedure.md` | Contract amendment |
| Conditional | `docs/specifications/science-contracts/AGENTS.md` | Instructions governing `SC-PLANT-001` edits |
| Conditional | `docs/specifications/science-contracts/kernel-process-contract-profile.md` | Kernel contract profile |
| Conditional | `docs/specifications/science-contracts/index.md` | Current authority registry |
| Conditional | `docs/standards/kernel-work-package-preparation.md` | Kernel package obligations |
| Conditional | `docs/standards/testing-and-gate-strategy.md` | Critical gate lifecycle |
| Conditional | `crates/AGENTS.md` | Instructions governing runner/orchestrator edits |
| On-demand | `docs/specifications/science-contracts/contracts/SC-PLANT-001.md` | Height-law adjudication and amendment |
| On-demand | applicable downstream `SC-*` contracts | Consumer proof phase |
| On-demand | CAL-04B incident 004 and retained attempt root | Exact reproduction |
| On-demand | touched Rust modules and nearest `AGENTS.md` files | Before edits |
| On-demand | pinned legacy canopy-height sources | Authority/provenance adjudication |

The exact fixed local Core + Conditional set, including the current package,
totals 588,326 bytes. That is above 400,000 and below 800,000 bytes: `WARN`.
On-demand sources are progressively disclosed at their triggers.

## Instruction Discovery

Ran:

`tools/agents/find-agents --for docs/specifications/science-contracts/contracts/SC-PLANT-001.md crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs crates/openwepp-runner/src/hillslope/tests03 crates/openwepp-hillslope-orchestrator/src/direct_runtime/growth.rs crates/openwepp-hillslope-orchestrator/src/direct_runtime/laned_active.rs docs/work-packages/20260727-cal04b-native-gsi-canopy-height-coherence-hold-lift-001/package.md`

Applicable instruction chain:

- root `AGENTS.md`
- `docs/specifications/science-contracts/AGENTS.md`
- `crates/AGENTS.md`
- `docs/work-packages/AGENTS.md`

All four were read before package or production edits. The package also uses
the local docs-maintainer workflow for Markdown validation; openWEPP's own
Markdown tooling remains the validation authority.
