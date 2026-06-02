Scope: local repository diagnostic/kernel-parity task; flat-file reads/edits
only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260602-hphys0253-h1-day1-storage-localization-diagnostics-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-EVAP-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260602-hphys0252-wb19-lateral-storage-availability-closure-001/artifacts/hphys0252_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260602-hphys0252-wb19-lateral-storage-availability-closure-001/artifacts/review_claude_code_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260602-hphys0252-wb19-lateral-storage-availability-closure-001/artifacts/worker-handoff.md`

Files:
- `docs/work-packages/20260602-hphys0253-h1-day1-storage-localization-diagnostics-001/**`
- `docs/work-packages/README.md`

Task: execute the diagnostic-only HPHYS0253 objective end-to-end for H1
t=0/day-1 storage localization and full `H1..H39` metric snapshot.

Constraints: no production code edits; no heuristic/proxy physics; canonical
SC authority remains the implementation authority for follow-on work; pinned
baseline provenance is `/workdir/wepp-forest_260430_baseline` commit
`dac3c950d8b16cc73774bf5ce2e7e11f80baac70`; typed guards; no silent defaults.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases,
including H1 localization, full `H1..H39` metrics, review artifacts,
verification artifacts, gate results, and continuation handoff.
