Scope: local repository science-contract/kernel diagnostics task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260531-hphys0217-post-0216d-coupled-family-rerun-readjudication-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/work-packages/20260531-hphys0216d-profilefc-normalized-tail-authority-reconciliation-001/artifacts/worker-handoff.md`
- `/tmp/unpalatable_parity_20260529T192707Z/reports/hillslope/baseline_partitions/`

Files:
- `docs/work-packages/README.md`
- `docs/work-packages/20260531-hphys0217-post-0216d-coupled-family-rerun-readjudication-001/**`
- `tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py`
- `/tmp/unpalatable_parity_20260529T192707Z/runs/`
- `/tmp/unpalatable_parity_20260529T192707Z/reports/hillslope/baseline_partitions/`

Task: execute HPHYS0217 end-to-end:
- run fresh 39-hillslope candidate lane for unpalatable-rind using current
  workspace binary and existing runfiles,
- run semantic comparator per hillslope against baseline partitions,
- publish monitored-family summary + integrated hold-lift disposition and
  follow-on queue.

Constraints:
- no production kernel/runtime code edits in this package,
- no canonical contract amendments in this package,
- preserve typed failure posture; do not introduce silent defaults,
- if residuals remain, publish explicit follow-on package boundaries.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs:
- update package artifacts/disposition for all completed phases,
- include truthful `Static:` vs `Ran:` evidence labels,
- publish concrete next-package recommendation (`HPHYS0218+`) if required.
