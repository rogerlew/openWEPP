Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260531-hphys0220-wb19-coupled-flux-partition-diagnostics-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/work-packages/20260531-hphys0219-wb19-coca-threshold-authority-correction-001/artifacts/worker-handoff.md`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for`

Files:
- `docs/work-packages/README.md`
- `docs/work-packages/20260531-hphys0220-wb19-coupled-flux-partition-diagnostics-001/**`

Task: execute HPHYS0220 diagnostics/remediation-planning objective end-to-end:
- quantify HPHYS0218 vs HPHYS0219 coupled residual directionality,
- map WB19 baseline process-lineage surfaces not represented in openWEPP,
- publish contract-first remediation handoff package scope.

Constraints:
- no production kernel/runtime edits in this package,
- no silent defaults/clamping recommendations,
- preserve canonical SC authority posture and baseline provenance references,
- maintain truthful `Static:`/`Ran:` evidence labels.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs:
- update package artifacts/disposition for all completed phases,
- publish concrete immediate-next package recommendation with explicit closure
  measures.
