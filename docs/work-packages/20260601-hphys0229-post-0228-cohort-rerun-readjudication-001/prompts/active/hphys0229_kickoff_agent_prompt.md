Scope: local repository science-contract/kernel migration task; flat-file
reads/edits only; no external connectivity.

Execution mode: package-end-to-end (default).

Phase plan: execute all phases in package.md sequentially through disposition.

Required reading (read before edits):
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0229-post-0228-cohort-rerun-readjudication-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0228-wb14-ksatadj-success-lane-restoration-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py`
- `/tmp/hphys0224_20260601T054337Z/parity/reports/hillslope_semantic_summary.json`
- `/tmp/unpalatable_parity_20260529T192707Z/reports/hillslope/baseline_partitions/`

Files:
- `docs/work-packages/README.md`
- `docs/work-packages/20260601-hphys0229-post-0228-cohort-rerun-readjudication-001/**`

Task: execute HPHYS0229 end-to-end as a diagnostics/readjudication package:
rerun `unpalatable-rind` hillslopes `H1..H39`, recompute semantic parity
reports/summaries, publish monitored-family deltas versus HPHYS0224
(`Dp`, `latqcc`, `Total-Soil`, `SoilWaterTotal`, `ProfileFCStore`), run
guardrail suites, and publish disposition/handoff.

Constraints: no production kernel/code edits; maintain typed-guard posture;
execute with valid semantic settings (`--candidate-year-offset 2012`, no
partition filter); preserve WB14 and required Level-4 suite hard-fail lanes.

Autonomy: execute package phases end-to-end and update required artifacts
without requesting additional user direction unless hard-blocked.

Outputs: update package artifacts/disposition for all completed phases.
