# SNOWPLAN01 Kickoff Agent Prompt

Scope: local repository planning/governance task; flat-file reads/edits only;
no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260525-snowplan01-hourly-energy-balance-snow-assessment-and-queue-001/package.md`
- `/workdir/openWEPP/docs/audits/20260525_water_erosion_kernel_audit.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`

Files:
- `docs/work-packages/20260525-snowplan01-hourly-energy-balance-snow-assessment-and-queue-001/package.md`
- `docs/work-packages/20260525-snowplan01-hourly-energy-balance-snow-assessment-and-queue-001/artifacts/*.md`
- `docs/work-packages/README.md`

Task: assess single-package feasibility for hourly energy-balance snow closure
and prepare a dependency-ordered four-package queue for execution.
Constraints: contract-first sequencing; canonical `SC-*` authority; baseline
provenance; typed guards; no silent defaults.
Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.
Outputs: update package artifacts/disposition for all completed phases.

Mandatory sequencing and governance gates:
- Do not modify production kernel/runtime code in this planning package.
- Queue items must encode internal contract sequencing:
  1. contracts,
  2. contract-derived tests,
  3. pre-implementation contract gate,
  4. production edits.
- Record `Static:` vs `Ran:` labels in all evidence artifacts.
