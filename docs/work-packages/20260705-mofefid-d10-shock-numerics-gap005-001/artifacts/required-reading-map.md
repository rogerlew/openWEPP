# Required Reading Map

Status: queued
Evidence mode: not-run

## Budget

| Field | Value |
|---|---|
| Local core + triggered contract/kernel conditional pre-edit bytes | `334130` |
| Threshold | `OK <= 400000`; `WARN > 400000`; `REQUIRES-JUSTIFICATION > 800000` |
| Disposition | `OK` |

## Map

| Path | Tier | Rationale | Trigger | Phase | Owner | Date read | Notes |
|---|---|---|---|---|---|---|---|
| `/home/workdir/openWEPP/AGENTS.md` | Core | Root governance for all package work | Always | Pre-edit | Agent | pending | |
| `/home/workdir/openWEPP/docs/codex_exec_plans.md` | Core | Execution-plan and review/disposition contract | Always | Pre-edit | Agent | pending | |
| `/home/workdir/openWEPP/docs/defect_closure_execplans.md` | Core | DC-ExecPlan conversion and HOLD legitimacy rules | Always | Pre-edit | Agent | pending | |
| `/home/workdir/openWEPP/docs/work-packages/AGENTS.md` | Core | Work-package gates, reviews, evidence rules | Always | Pre-edit | Agent | pending | |
| `/home/workdir/openWEPP/docs/work-packages/README.md` | Core | Package discovery and current active/held status | Always | Pre-edit | Agent | pending | |
| `/home/workdir/openWEPP/docs/work-packages/20260705-mofefid-d10-shock-numerics-gap005-001/package.md` | Core | Package-local objective, scope, envelope, gates | Always | Pre-edit | Agent | pending | |
| `/home/workdir/openWEPP/docs/specifications/science-contracts/AGENTS.md` | Conditional | Canonical `SC-*` editing governance | Contract/kernel authority edit | Pre-edit when triggered | Agent | pending | |
| `/home/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md` | Conditional | Contract-authoring procedure | Contract edit | Pre-edit when triggered | Agent | pending | |
| `/home/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md` | Conditional | Kernel-process profile compliance | Contract or kernel authority edit | Pre-edit when triggered | Agent | pending | |
| `/home/workdir/openWEPP/docs/specifications/science-contracts/index.md` | Conditional | Contract registry lifecycle | Registry/profile status edit | Pre-edit when triggered | Agent | pending | |
| `/home/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md` | Conditional | Pinned-baseline provenance governance | Pinned legacy source use | Pre-edit when triggered | Agent | pending | |
| `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` | On-demand | Canonical Lane D contract and `GAP-OFEROUTE-005` authority | Contract amendment or status read | Phase-local | Agent | pending | |
| `/home/workdir/openWEPP/docs/planning/mofe-fidelity-campaign-strategy.md` | On-demand | D10-D15 sequencing and activation boundaries | Scope/status check | Phase-local | Agent | pending | |
| `/home/workdir/openWEPP/docs/work-packages/20260705-mofefid-d9-dval-disposition-001/artifacts/case4-d10-handoff.md` | On-demand | D9 exact D10 handoff | Intake | Phase-local | Agent | pending | |
| `/home/workdir/openWEPP/docs/work-packages/20260702-mofefid-d8-routing-fidelity-defect-closure-001/artifacts/execution-report.md` | On-demand | D8 Case-4 baseline and `GAP-OFEROUTE-005` opening evidence | Intake/baseline | Phase-local | Agent | pending | |
| `/home/workdir/openWEPP/docs/work-packages/20260705-mofefid-laned-activation-increment-001/package.md` | On-demand | H2637 shadow reproduction context | H2637 evidence | Phase-local | Agent | pending | |
| `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/ofe_routing/kinematic_wave.rs` | On-demand | Current KWE/TVD solver implementation | Solver inspection/edit | Phase-local | Agent | pending | |
| `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/ofe_routing/cascade.rs` | On-demand | OFE-by-OFE handoff and sampled cascade behavior | Cascade inspection/edit | Phase-local | Agent | pending | |
| `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/ofe_routing/infiltration.rs` | On-demand | Rainfall-to-excess cascade wrapper and controls | Wrapper inspection/edit | Phase-local | Agent | pending | |
| `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/ofe_routing/dval.rs` | On-demand | D-val Case 4 setup and metrics | D-val inspection/edit | Phase-local | Agent | pending | |
| `/home/workdir/openWEPP/tools/dval/compare_dval.py` | On-demand | D-val metric harness | Harness inspection/edit | Phase-local | Agent | pending | |
| `references/copyrighted/Iwagaki1955_runoff_characteristics_DPRI10.pdf` | On-demand | Case-4 shock primary/source evidence | Source provenance | Phase-local | Agent | pending | Copyright governance applies. |
| `references/copyrighted/Papanicolaou2018.md` | On-demand | Lane D equation/source context | Source provenance | Phase-local | Agent | pending | Copyright governance applies. |
| `references/copyrighted/Papanicolaou2018-supplemental/` | On-demand | Case-4 supplemental-derived validation data | D-val evidence | Phase-local | Agent | pending | Do not vendor raw workbook rows. |
| `references/copyrighted/mingham2001.pdf` | On-demand | TVD/shock numerical-method source authority candidate | Source provenance | Phase-local | Agent | pending | Copyright governance applies. |
