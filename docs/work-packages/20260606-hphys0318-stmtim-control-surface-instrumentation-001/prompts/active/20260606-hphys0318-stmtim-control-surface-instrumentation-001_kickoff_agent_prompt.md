# HPHYS0318 Kickoff Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260606-hphys0318-stmtim-control-surface-instrumentation-001/package.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `docs/decisions/0017-re-pin-operational-distrust-comparator-is-flag-not-target.md`
- `docs/work-packages/20260606-hphys0317-paired-hourly-snowfall-input-surface-parity-001/artifacts/worker-handoff.md`

Files:

- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/index.md`
- `Cargo.toml`
- `crates/openwepp-sim-contract/src/units.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/06_simimpl28_hourly_forcing.rs`
- `crates/openwepp-hillslope-orchestrator/src/runtime_inputs/08_tests.rs`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `tests/integration/sim_contract_boundary_unit_registry.rs`
- `tests/integration/hphys0318_stmtim_control_surface_instrumentation_contract.rs`
- `docs/work-packages/README.md`
- `docs/work-packages/20260606-hphys0318-stmtim-control-surface-instrumentation-001/**`

Task: execute package objective end-to-end for declared scope. Add
contract-backed OpenWEPP `snow.hourly.stmtim.*` trace instrumentation for
`rain`, `stmdur`, `wntdur`, `wnttim`, `hrtemp`, `rst`, `hrrain`, `hrsnow`,
active interval membership, and branch choice, while preserving the current
partition results and route hold.

Constraints: contract-first sequencing; canonical SC authority; baseline
provenance from `/workdir/wepp-forest_260430_baseline`; typed guards; no silent
defaults; no canonicalize-and-proceed for domain violations; no production
precipitation-phase physics edit; no downstream WB13/WB17/WB18/WB19/WB12
compensation.

Constraint phrase: no production precipitation-phase physics edit.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.
