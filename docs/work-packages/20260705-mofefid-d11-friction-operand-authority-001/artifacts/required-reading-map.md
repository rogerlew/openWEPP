# Required Reading Map

Status: executed
Evidence mode: Static

## Budget

| Field | Value |
|---|---|
| Local core + triggered contract/kernel conditional pre-edit bytes | `307265` |
| Threshold | `OK <= 400000`; `WARN > 400000`; `REQUIRES-JUSTIFICATION > 800000` |
| Disposition | `OK` |

## Map

| Path | Tier | Rationale | Trigger | Phase | Owner | Date read | Notes |
|---|---|---|---|---|---|---|---|
| `/home/workdir/openWEPP/AGENTS.md` | Core | Root governance for all package work | Always | Pre-edit | Codex | 2026-07-06 | Read. |
| `/home/workdir/openWEPP/docs/codex_exec_plans.md` | Core | Execution-plan and review/disposition contract | Always | Pre-edit | Codex | 2026-07-06 | Read. |
| `/home/workdir/openWEPP/docs/work-packages/AGENTS.md` | Core | Work-package gates, reviews, evidence rules | Always | Pre-edit | Codex | 2026-07-06 | Read. |
| `/home/workdir/openWEPP/docs/work-packages/README.md` | Core | Package discovery and current active/held status | Always | Pre-edit | Codex | 2026-07-06 | Read and updated. |
| `/home/workdir/openWEPP/docs/work-packages/20260705-mofefid-d11-friction-operand-authority-001/package.md` | Core | Package-local objective, scope, gates | Always | Pre-edit | Codex | 2026-07-06 | Read and updated. |
| `/home/workdir/openWEPP/docs/specifications/science-contracts/AGENTS.md` | Conditional | Canonical `SC-*` editing governance | Contract/kernel authority edit | Pre-edit | Codex | 2026-07-06 | Read before `SC-OFEROUTE-001` edit. |
| `/home/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md` | Conditional | Contract-authoring procedure | Contract edit | Pre-edit | Codex | 2026-07-06 | Read before `SC-OFEROUTE-001` edit. |
| `/home/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md` | Conditional | Kernel-process profile compliance | Contract or kernel authority edit | Pre-edit | Codex | 2026-07-06 | Read before `SC-OFEROUTE-001` edit. |
| `/home/workdir/openWEPP/docs/specifications/science-contracts/index.md` | Conditional | Contract registry lifecycle | Registry/profile status edit | Pre-edit | Codex | 2026-07-06 | Read; registry status unchanged. |
| `/home/workdir/openWEPP/docs/specifications/unit-governance.md` | Conditional | Unit/alias/default governance | Unit and symbol map update | Pre-edit | Codex | 2026-07-06 | Read before adding friction operand unit rows. |
| `/home/workdir/openWEPP/docs/specifications/correctness-authority-model.md` | Conditional | HOLD and authority ranking | Source-authority hold | Pre-edit | Codex | 2026-07-06 | Read. |
| `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-OFEROUTE-001.md` | On-demand | Canonical Lane D routing contract and `GAP-OFEROUTE-007` | Contract amendment/status read | S0-S2 | Codex | 2026-07-06 | Read and amended to rev 19. |
| `/home/workdir/openWEPP/docs/planning/mofe-fidelity-campaign-strategy.md` | On-demand | D11-D16 sequencing and activation boundaries | Scope/status check | S0/S5 | Codex | 2026-07-06 | Read and updated. |
| `/home/workdir/openWEPP/docs/work-packages/20260705-mofefid-d10-shock-numerics-gap005-001/artifacts/worker-handoff.md` | On-demand | Case-4 boundary and D10 follow-on separation | Boundary check | S0 | Codex | 2026-07-06 | Read. |
| `/home/workdir/openWEPP/docs/work-packages/20260705-mofefid-laned-activation-increment-001/package.md` | On-demand | Runtime shadow status and H2637 vector | Builder/fixture context | S0/S1 | Codex | 2026-07-06 | Read. |
| `/home/workdir/openWEPP/crates/openwepp-runner/src/hillslope/laned_shadow.rs` | On-demand | Current hardcoded friction consumer | Builder inspection | S0/S1 | Codex + explorer | 2026-07-06 | Read. |
| `/home/workdir/openWEPP/crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00_builders_and_authority.rs` | On-demand | Candidate static/daily source projections | Source audit | S1 | Codex + explorer | 2026-07-06 | Read. |
| `/home/workdir/openWEPP/crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/00c_day_input_builder_impl.rs` | On-demand | Hyetograph/growth publication path | Source audit | S1 | Codex | 2026-07-06 | Read. |
| `/home/workdir/openWEPP/crates/openwepp-runner/src/hillslope/direct_publication/day_input_and_helpers/01_frost_and_layer_helpers.rs` | On-demand | Typed hyetograph builder and rainfall-scale guards | Source audit | S1 | Codex | 2026-07-06 | Read. |
| `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/ofe_routing/` | On-demand | Friction kernels and cascade consumer | Source audit | S1 | Codex + explorer | 2026-07-06 | Read. |
| `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md` | On-demand | Rainfall-intensity source authority | `I` audit | S1 | Codex + explorer | 2026-07-06 | Read relevant WB14 addendum. |
| `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PLANT-001.md` | On-demand | `LAI` / `Hc` source authority | vegetation audit | S1 | Codex + explorer | 2026-07-06 | Read relevant variables/obligations. |
| `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md` | On-demand | Adjacent roughness authority check | roughness audit | S1 | Codex + explorer | 2026-07-06 | Read; not a direct alias. |
| `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RESIDUE-001.md` | On-demand | Residue-depth candidate rejection | roughness audit | S1 | Codex | 2026-07-06 | Read; no Papanicolaou roughness mapping. |

`docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md` was not triggered:
D11 did not read new pinned baseline source or add legacy-source-derived physics.
