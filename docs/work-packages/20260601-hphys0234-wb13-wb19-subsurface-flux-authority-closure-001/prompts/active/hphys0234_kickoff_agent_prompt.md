Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.
Execution mode: package-end-to-end (default).
Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0234-wb13-wb19-subsurface-flux-authority-closure-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0233-wb18-daily-restrictive-conductivity-authority-closure-001/artifacts/hphys0233_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0233-wb18-daily-restrictive-conductivity-authority-closure-001/artifacts/worker-handoff.md`

Files:
- `docs/work-packages/README.md`
- `docs/work-packages/20260601-hphys0234-wb13-wb19-subsurface-flux-authority-closure-001/**`
- `docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `crates/openwepp-runner/src/hillslope/mod.rs`

Task: execute HPHYS0234 objective end-to-end for declared scope: close WB13
subsurface publication anti-shadow lineage by making `latqcc`/`Tile`/`Qd`
publication and coupling checks flux-authoritative (`q`/`Qdd`/`Qd`), rerun
`unpalatable-rind` (`H1..H39`), and publish readjudication/disposition.

Constraints: contract-first sequencing; canonical SC authority; baseline
provenance (`/workdir/wepp-forest_260430_baseline`); typed guards; no silent
defaults for domain violations; no heuristic/proxy process-physics
substitutions in production paths.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.
