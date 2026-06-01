Scope: local repository science-contract/kernel migration discovery task;
flat-file reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0237-hourly-bulk-routine-inventory-and-dispatch-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0236-wb18-hourly-iterative-execution-closure-001/artifacts/hphys0236_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0236-wb18-hourly-iterative-execution-closure-001/artifacts/worker-handoff.md`
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for`
- `/workdir/wepp-forest_260430_baseline/src/purk.for`
- `/workdir/wepp-forest_260430_baseline/src/drain.for`

Files:
- `docs/work-packages/README.md`
- `docs/work-packages/20260601-hphys0237-hourly-bulk-routine-inventory-and-dispatch-001/**`

Task: execute HPHYS0237 discovery objective end-to-end. Produce a complete
hourly-routine inventory and dispatch queue covering all baseline-authoritative
hydrology routines still requiring iterative substep migration in openWEPP.

Constraints: contract-first sequencing; canonical SC authority; baseline
provenance; typed guards; no silent defaults; no heuristic/proxy process-physics
substitutions.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update routine-inventory artifact, queue mapping, disposition, and
handoff artifacts.
