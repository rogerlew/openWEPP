Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0235-wb18-dp-7x-legacy-root-cause-closure-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0234-wb13-wb19-subsurface-flux-authority-closure-001/artifacts/hphys0234_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0234-wb13-wb19-subsurface-flux-authority-closure-001/artifacts/worker-handoff.md`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for`
- `/workdir/wepp-forest_260430_baseline/src/watbal_hourly.for`
- `/workdir/wepp-forest_260430_baseline/src/purk.for`
- `/workdir/wepp-forest_260430_baseline/src/perc.for`

Files:
- `docs/work-packages/README.md`
- `docs/work-packages/20260601-hphys0235-wb18-dp-7x-legacy-root-cause-closure-001/**`
- `docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`

Task: execute HPHYS0235 objective end-to-end for declared scope: isolate and
evidence why early `H1` `Dp` remains `~7x` legacy under `ui_run=1`, amend
canonical WB18 hourly-lane authority to reflect baseline `watbal_hourly` +
`purk` iterative semantics, and publish implementation-ready hold handoff.

Constraints: contract-first sequencing; canonical SC authority; baseline
provenance (`/workdir/wepp-forest_260430_baseline`); typed guards; no silent
defaults for domain violations; no heuristic/proxy process-physics
substitutions in production paths.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.
