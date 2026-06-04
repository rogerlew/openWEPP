# HPHYS0287 Kickoff Agent Prompt

Scope: local repository science-contract/kernel migration task; flat-file reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in `package.md` sequentially through disposition.

Required reading:
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260604-hphys0287-snow-liquid-retention-runoff-infiltration-partition-closure-001/package.md`
- `docs/specifications/science-contract-authoring-procedure.md`
- `docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `docs/work-packages/20260604-hphys0286-layer-retention-wb18-wb17-coupling-closure-001/artifacts/worker-handoff.md`
- `docs/work-packages/20260604-hphys0286-layer-retention-wb18-wb17-coupling-closure-001/artifacts/full-39-suite-metrics.md`
- `docs/work-packages/20260604-hphys0286-layer-retention-wb18-wb17-coupling-closure-001/artifacts/disposition.md`

Files:
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/**`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `tests/integration/hphys0287_snow_liquid_partition_guard_contract.rs`
- `Cargo.toml`
- `docs/work-packages/20260604-hphys0287-snow-liquid-retention-runoff-infiltration-partition-closure-001/**`

Task: execute the package objective end-to-end for declared scope.

Constraints: contract-first sequencing; canonical SC authority; baseline provenance; typed guards; no silent defaults; no canonicalize-and-proceed for domain violations; no heuristic/proxy process-physics substitutions.

Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.
