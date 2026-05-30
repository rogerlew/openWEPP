Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260530-hphys0211-coupled-threshold-root-cause-ledger-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260530-hphys0208-fc-threshold-coupled-residual-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260530-hphys0209-profilewp-near-closed-adjudication-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260530-hphys0210-integrated-hold-lift-adjudication-001/artifacts/hphys0210_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260530-hphys0210-integrated-hold-lift-adjudication-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`

Files:
- `docs/work-packages/README.md`
- `docs/work-packages/20260530-hphys0211-coupled-threshold-root-cause-ledger-001/**`

Task: execute package objective end-to-end for declared scope: close semantic
root-cause decomposition for coupled-threshold residual families
(`ProfileFCStore`, `Dp`, `latqcc`, `Total-Soil`, `SoilWaterTotal`) with
process-authority-first disposition logic and scoped remediation handoff.

Constraints: contract-first sequencing; canonical SC authority; typed guards;
no silent defaults; baseline provenance anchored to
`wepp-forest_260430_baseline` commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`;
no heuristic/proxy physics substitutions in production code.
Do not implement production kernel features in this package; this package is
decomposition/evidence only.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.
