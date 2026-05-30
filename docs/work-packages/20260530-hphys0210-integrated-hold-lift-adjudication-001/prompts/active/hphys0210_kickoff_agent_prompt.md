Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260530-hphys0210-integrated-hold-lift-adjudication-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260530-hphys0208-fc-threshold-coupled-residual-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260530-hphys0209-profilewp-near-closed-adjudication-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260529-hphys0204-disposition-and-diagnostics-001/artifacts/hphys0204_disposition.md`
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
- `docs/work-packages/20260530-hphys0210-integrated-hold-lift-adjudication-001/**`
- `docs/specifications/science-contracts/index.md` (only if adjudication
  registry updates are required)
- `tools/legacy_comparison_suite/**` (only if diagnostic formatting updates are
  required)

Task: execute package objective end-to-end for declared scope: close semantic
integrated hold-lift adjudication for the HPHYS residual lanes after HPHYS0208
and HPHYS0209, with process-authority-first disposition logic.

Constraints: contract-first sequencing; canonical SC authority; typed guards;
no silent defaults; baseline provenance anchored to
`wepp-forest_260430_baseline` commit `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`;
no heuristic/proxy physics substitutions in production code.
Do not implement new production kernel features in this package unless required
by hard evidence and explicit in-scope contract/test/gate sequencing.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.
