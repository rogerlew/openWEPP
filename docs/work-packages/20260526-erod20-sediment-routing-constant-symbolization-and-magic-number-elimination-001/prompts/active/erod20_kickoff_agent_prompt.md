# EROD20 Kickoff Agent Prompt

Scope: local repository science-contract/kernel migration task; flat-file reads/edits only; no external connectivity.
Execution mode: package-end-to-end.
Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260526-erod20-sediment-routing-constant-symbolization-and-magic-number-elimination-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `/workdir/openWEPP/docs/work-packages/20260526-routeplan01-hillslope-sediment-routing-assessment-and-queue-001/artifacts/sediment-routing-wp-queue.md`

Files:
- `crates/openwepp-hillslope-orchestrator/src/constants.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support.rs`
- `docs/work-packages/20260526-erod20-sediment-routing-constant-symbolization-and-magic-number-elimination-001/**`

Task: execute EROD20 objective end-to-end for declared scope.

Constraints: contract-first sequencing; canonical SC authority; baseline provenance (`/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`); typed guards; no silent defaults.

Autonomy: execute package phases end-to-end and update required artifacts without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.
