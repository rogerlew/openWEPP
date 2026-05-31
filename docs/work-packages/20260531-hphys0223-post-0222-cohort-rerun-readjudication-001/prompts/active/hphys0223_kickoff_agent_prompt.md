Scope: local repository diagnostics task; flat-file reads/edits only; no
external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in `package.md` sequentially through
disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260531-hphys0223-post-0222-cohort-rerun-readjudication-001/package.md`
- `/workdir/openWEPP/tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py`
- `/tmp/hphys0221_20260531T141839Z/parity/reports/hillslope_semantic_summary.json`

Files:
- `docs/work-packages/README.md`
- `docs/work-packages/20260531-hphys0223-post-0222-cohort-rerun-readjudication-001/**`

Task: execute HPHYS0223 rerun/readjudication end-to-end:
1. rerun `H1..H39` hillslope outputs for `unpalatable-rind`,
2. rerun semantic comparator with valid row-key alignment,
3. aggregate monitored-family summaries,
4. compare against HPHYS0221 and publish disposition.

Constraints:
- No production code edits in this package.
- Truthful evidence only (`Static:` vs `Ran:` labels).
- Explicitly record any failed first-attempt run and recovery rerun.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs:
- updated diagnostics artifacts,
- rerun summary references,
- HOLD/GO disposition with explicit next-step trigger.
