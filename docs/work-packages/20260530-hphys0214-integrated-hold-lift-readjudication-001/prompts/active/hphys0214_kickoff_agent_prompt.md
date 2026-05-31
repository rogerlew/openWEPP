Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260530-hphys0214-integrated-hold-lift-readjudication-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/work-packages/20260530-hphys0211-coupled-threshold-root-cause-ledger-001/artifacts/hphys0211_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260530-hphys0212-wb11-wb19-lifecycle-coupling-closure-001/artifacts/hphys0212_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260530-hphys0213-wb12-storage-and-aggregate-reconciliation-closure-001/artifacts/hphys0213_disposition.md`

Files:
- `docs/work-packages/20260530-hphys0214-integrated-hold-lift-readjudication-001/**`
- `docs/work-packages/README.md`

Task: execute HPHYS0214 end-to-end for declared scope:
- perform integrated hold-lift readjudication after HPHYS0211/0212/0213,
- rerun required workspace gates and publish reproducible evidence,
- recompute/publish integrated monitored-family diagnostics for final
  process-authority-first `HOLD`/`GO` disposition.

Constraints:
- contract-first sequencing (contracts -> contract-derived tests ->
  pre-implementation gate -> production edits),
- canonical SC authority updates when required by changed obligations,
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
- publish explicit hold-lift posture and immediate-next queue.
