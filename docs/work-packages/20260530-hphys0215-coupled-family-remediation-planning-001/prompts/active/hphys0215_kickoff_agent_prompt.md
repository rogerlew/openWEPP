Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260530-hphys0215-coupled-family-remediation-planning-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/work-packages/20260530-hphys0214-integrated-hold-lift-readjudication-001/artifacts/hphys0214_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260530-hphys0214-integrated-hold-lift-readjudication-001/artifacts/hphys0214-residual-gap-matrix.md`
- `/workdir/openWEPP/docs/work-packages/20260530-hphys0214-integrated-hold-lift-readjudication-001/artifacts/worker-handoff.md`

Files:
- `docs/work-packages/20260530-hphys0215-coupled-family-remediation-planning-001/**`
- `docs/work-packages/README.md`

Task: execute HPHYS0215 end-to-end for declared scope:
- produce the coupled-family remediation stream plan for remaining blockers
  (`ProfileFCStore`, `Dp`, `latqcc`, `Total-Soil`, `SoilWaterTotal`),
- define bounded HPHYS0216+ implementation queue with contract-first
  sequencing, owners, and closure measures,
- run required workspace gates and publish truthful evidence.

Constraints:
- contract-first sequencing (contracts -> contract-derived tests ->
  pre-implementation gate -> production edits),
- canonical SC authority governs all remediation obligations,
- baseline provenance anchor
  (`/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`)
  for migration lineage,
- typed guards; no silent defaults/clamping for domain violations,
- no heuristic/proxy physics substitutions,
- dual review + dual verification artifacts required.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs:
- update package artifacts/disposition for all completed phases,
- include truthful `Static:` vs `Ran:` evidence labels,
- publish actionable HPHYS0216+ execution queue.
