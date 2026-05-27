# REFACTOR005 Kickoff Agent Prompt

Scope: local repository science-contract/kernel migration task; flat-file reads/edits only; no external connectivity.
Execution mode: package-end-to-end.
Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260526-refactor005-openwepp-hillslope-orchestrator-kernel-support-mechanical-modularization-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/work-packages/20260525-refactor003-openwepp-hillslope-orchestrator-hydrology-mechanical-modularization-001/package.md`

Files:
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support_*.rs`
- `docs/work-packages/20260526-refactor005-openwepp-hillslope-orchestrator-kernel-support-mechanical-modularization-001/**`

Task: execute REFACTOR005 objective end-to-end for declared scope.

Constraints: contract-first sequencing; canonical SC authority; typed guards; no silent defaults; do not edit production kernel code before pre-implementation contract-gate evidence is recorded.

Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.
