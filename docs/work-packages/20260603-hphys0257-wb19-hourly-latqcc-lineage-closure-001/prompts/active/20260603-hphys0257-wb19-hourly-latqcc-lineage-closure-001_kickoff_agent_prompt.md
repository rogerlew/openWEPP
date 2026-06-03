# HPHYS0257 Kickoff Agent Prompt

Scope: local repository science-contract/kernel migration task; flat-file reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):

- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260603-hphys0257-wb19-hourly-latqcc-lineage-closure-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260602-hphys0256-wb19-latqcc-lane-branch-closure-001/artifacts/disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260602-hphys0256-wb19-latqcc-lane-branch-closure-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/work-packages/20260602-hphys0256-wb19-latqcc-lane-branch-closure-001/artifacts/targeted-h1-h7-h39-diagnostics.md`
- `/workdir/openWEPP/docs/work-packages/20260602-hphys0256-wb19-latqcc-lane-branch-closure-001/artifacts/full-39-suite-metrics.md`
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for`

Files:

- `docs/work-packages/20260603-hphys0257-wb19-hourly-latqcc-lineage-closure-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `Cargo.toml`
- `tests/integration/hphys0257_wb19_hourly_latqcc_lineage_contract.rs`

Task: execute package objective end-to-end for declared scope.

Constraints: contract-first sequencing; canonical SC authority; baseline provenance from `/workdir/wepp-forest_260430_baseline` commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`; typed guards; no silent defaults; no heuristic or proxy process-physics substitutions.

Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.
