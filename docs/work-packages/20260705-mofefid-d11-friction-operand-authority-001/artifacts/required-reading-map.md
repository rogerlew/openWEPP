# Required Reading Map

Status: queued
Evidence mode: not-run

## Budget

| Field | Value |
|---|---|
| Local core + triggered contract/kernel conditional pre-edit bytes | `307265` |
| Threshold | `OK <= 400000`; `WARN > 400000`; `REQUIRES-JUSTIFICATION > 800000` |
| Disposition | `OK` |

## Map

| Path | Tier | Rationale | Trigger | Phase | Owner | Date read | Notes |
|---|---|---|---|---|---|---|---|
| `/home/workdir/openWEPP/AGENTS.md` | Core | Root governance for all package work | Always | Pre-edit | Agent | pending | |
| `/home/workdir/openWEPP/docs/codex_exec_plans.md` | Core | Execution-plan and review/disposition contract | Always | Pre-edit | Agent | pending | |
| `/home/workdir/openWEPP/docs/work-packages/AGENTS.md` | Core | Work-package gates, reviews, evidence rules | Always | Pre-edit | Agent | pending | |
| `/home/workdir/openWEPP/docs/work-packages/README.md` | Core | Package discovery and current active/held status | Always | Pre-edit | Agent | pending | |
| `/home/workdir/openWEPP/docs/work-packages/20260705-mofefid-d11-friction-operand-authority-001/package.md` | Core | Package-local objective, scope, gates | Always | Pre-edit | Agent | pending | |
| `/home/workdir/openWEPP/docs/specifications/science-contracts/AGENTS.md` | Conditional | Canonical `SC-*` editing governance | Contract/kernel authority edit | Pre-edit when triggered | Agent | pending | |
| `/home/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md` | Conditional | Contract-authoring procedure | Contract edit | Pre-edit when triggered | Agent | pending | |
| `/home/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md` | Conditional | Kernel-process profile compliance | Contract or kernel authority edit | Pre-edit when triggered | Agent | pending | |
| `/home/workdir/openWEPP/docs/specifications/science-contracts/index.md` | Conditional | Contract registry lifecycle | Registry/profile status edit | Pre-edit when triggered | Agent | pending | |
| `/home/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md` | Conditional | Pinned-baseline provenance governance | Pinned legacy source use | Pre-edit when triggered | Agent | pending | |
| `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` | On-demand | Canonical Lane D routing contract and `GAP-OFEROUTE-007` | Contract amendment/status read | Phase-local | Agent | pending | |
| `/home/workdir/openWEPP/docs/planning/mofe-fidelity-campaign-strategy.md` | On-demand | D11-D16 sequencing and activation boundaries | Scope/status check | Phase-local | Agent | pending | |
| `/home/workdir/openWEPP/docs/work-packages/20260705-mofefid-d10-shock-numerics-gap005-001/artifacts/worker-handoff.md` | On-demand | Case-4 boundary and D10 follow-on separation | Boundary check | Phase-local | Agent | pending | |
| `/home/workdir/openWEPP/docs/work-packages/20260705-mofefid-laned-activation-increment-001/package.md` | On-demand | Runtime shadow status and H2637 vector | Builder/fixture context | Phase-local | Agent | pending | |
| `/home/workdir/openWEPP/crates/openwepp-runner/src/hillslope/laned_shadow.rs` | On-demand | Current hardcoded friction consumer | Builder inspection/edit | Phase-local | Agent | pending | |
| `/home/workdir/openWEPP/crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs` | On-demand | Candidate static/daily source projections | Source audit/edit | Phase-local | Agent | pending | |
| `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/ofe_routing/` | On-demand | Friction kernels and cascade consumer | Kernel inspection/edit | Phase-local | Agent | pending | |
