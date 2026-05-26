# FROSTPLAN01 Kickoff Agent Prompt

Scope: local repository science-contract/kernel migration planning task;
flat-file reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.
Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260526-frostplan01-frost-energy-solver-assessment-and-queue-001/package.md`
- `/workdir/openWEPP/docs/audits/20260525_water_erosion_kernel_audit.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SNOWFREEZE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260525-simimpl30-winter-hourly-semantic-parity-rerun-and-disposition-001/artifacts/simimpl30-hold-lift-decision-report.md`

Files:
- `docs/work-packages/20260526-frostplan01-frost-energy-solver-assessment-and-queue-001/package.md`
- `docs/work-packages/20260526-frostplan01-frost-energy-solver-assessment-and-queue-001/artifacts/*.md`
- `docs/work-packages/20260526-frostplan01-frost-energy-solver-assessment-and-queue-001/prompts/active/frostplan01_kickoff_agent_prompt.md`
- `docs/work-packages/README.md`

Task: execute package objective end-to-end for declared scope.
Constraints: contract-first sequencing; canonical SC authority;
baseline provenance (`/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`);
typed guards; no silent defaults.
Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.
Outputs: update package artifacts/disposition for all completed phases.

Mandatory sequencing and governance gates:
- Do not modify production kernel/runtime code before contract + test + gate
  completion in follow-on code-authoring packages.
- For migration queue items, canonical `SC-*` physics authority updates are
  required before runtime implementation work.
- Prohibit silent defaults/clamping for domain violations; require typed
  errors/guards.
