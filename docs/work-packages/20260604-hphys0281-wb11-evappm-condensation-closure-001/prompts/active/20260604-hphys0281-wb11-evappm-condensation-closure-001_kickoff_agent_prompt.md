# HPHYS0281 Kickoff Agent Prompt

Scope: local repository science-contract/kernel migration task; flat-file reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- /workdir/openWEPP/AGENTS.md
- /workdir/openWEPP/docs/codex_exec_plans.md
- /workdir/openWEPP/docs/work-packages/README.md
- /workdir/openWEPP/docs/work-packages/20260604-hphys0281-wb11-evappm-condensation-closure-001/package.md
- /workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md
- /workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md
- /workdir/openWEPP/docs/specifications/science-contracts/index.md
- /workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-EVAP-001.md
- /workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md
- /workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md
- /workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md
- /workdir/openWEPP/docs/work-packages/20260604-hphys0280-hphys0275-typed-boundary-continuation-001/artifacts/wb11-et-e-003-characterization.md
- /workdir/openWEPP/docs/work-packages/20260604-hphys0280-hphys0275-typed-boundary-continuation-001/artifacts/worker-handoff.md
- /workdir/wepp-forest_260430_baseline/src/evappm.for
- /workdir/wepp-forest_260430_baseline/src/swu.for

Files:
- docs/specifications/science-contracts/contracts/SC-EVAP-001.md
- crates/openwepp-runner/src/hillslope/mod.rs
- tests/integration/pl14s_tier_a_candidate_emission_and_replay_contract.rs
- tests/integration/hphys0281_wb11_evappm_condensation_contract.rs
- Cargo.toml
- docs/work-packages/README.md
- docs/work-packages/20260604-hphys0281-wb11-evappm-condensation-closure-001/**

Task: execute package objective end-to-end for declared scope. Preserve the WB11 material-negative PMET guard. Diagnose and correct the EVAPPM seed producer so supersaturated cold-day condensation cases return negative `es - resint` demand to top-layer storage and publish non-negative `pmet.es_m` under canonical `SC-EVAP-001` authority.

Constraints: contract-first sequencing; canonical SC authority; baseline provenance from `/workdir/wepp-forest_260430_baseline` at `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`; typed guards; no silent defaults; no heuristic or proxy process-physics substitutions; no relaxing the WB11 guard; no climate input rejection as a substitute for baseline condensation handling; dual independent review and dual verification required.

Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases, including diagnostic evidence, gate results, review finding dispositions, verification artifacts, and worker handoff.
