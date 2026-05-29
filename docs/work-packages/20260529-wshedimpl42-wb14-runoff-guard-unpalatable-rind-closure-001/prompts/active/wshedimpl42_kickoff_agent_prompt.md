# WSHEDIMPL42 Kickoff Agent Prompt

Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in `package.md` sequentially through
disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260529-wshedimpl42-wb14-runoff-guard-unpalatable-rind-closure-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`

Files:
- `docs/work-packages/20260529-wshedimpl42-wb14-runoff-guard-unpalatable-rind-closure-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `docs/specifications/science-contracts/contracts/SC-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-runner/src/hillslope/mod.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_00_support_helpers.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_01_kernel_phases.rs`
- `tests/integration/wb14_infiltration_hyetograph_kernel_contract.rs`
- `tests/integration/cli03_runner_contract_derived_tests.rs`

Task: execute WSHEDIMPL42 end-to-end for declared scope by closing the active
`HKERNEL-WB14-RUNOFF-E-003` blocker on `/wc1/runs/un/unpalatable-rind` and
rerunning watershed output to parquet closure.

Constraints: contract-first sequencing; canonical SC authority; baseline
provenance (`/workdir/wepp-forest_260430_baseline` commit
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`); typed guards; no silent defaults;
no heuristic/proxy process-physics substitution.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases,
including gate results and explicit watershed rerun closure evidence.
