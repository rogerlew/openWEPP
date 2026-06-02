# 20260602-hphys0243-post-0242-39-hillslope-watershed-parity-readjudication-001

## Status
- state: completed
- date: 2026-06-02
- timezone: America/Los_Angeles
- decision: HOLD_PENDING_WB11_SNOW_STORAGE_WB18_COUPLED_CLOSURE

## Objective
Execute a fresh post-HPHYS0242 `unpalatable-rind` 39-hillslope plus watershed
rerun, assess semantic parity evidence, and review prior HPHYS residuals to
recommend the next remediation focus.

## Why This Package Exists
HPHYS0240..HPHYS0242 closed the immediate HPHYS0239 hourly carry/cadence
follow-up queue. The integrated `unpalatable-rind` cohort now needs fresh
execution evidence to determine whether the dominant residual family changed
and where implementation attention should focus next.

## Scope
### Included
- Build and execute `openwepp-cli-hill` for `H1..H39` using the existing
  `unpalatable-rind` openWEPP runfiles.
- Execute `openwepp-cli-watershed` using `pw0_openwepp.run` and the freshly
  generated HBP pass shards.
- Recompute hillslope semantic reports against baseline partitions with
  `--candidate-year-offset 2012`.
- Produce an investigation-grade watershed interchange comparison against
  `/wc1/runs/un/unpalatable-rind/wepp/output/interchange`.
- Compare current monitored-family summary metrics against prior HPHYS
  summaries.
- Review earlier HPHYS dispositions/residual matrices and publish focus
  recommendations.

### Explicitly Out of Scope
- Production code edits.
- Science-contract amendments.
- New process-physics implementation.
- Commit/push unless separately requested.

## Closure Measures
1. `MEASURE-HP243-001`: fresh `H1..H39` candidate executions complete with
   explicit pass/fail status.
2. `MEASURE-HP243-002`: fresh watershed execution completes or records a typed
   blocker.
3. `MEASURE-HP243-003`: hillslope semantic comparator artifacts have valid row
   overlap for all completed hillslopes.
4. `MEASURE-HP243-004`: watershed interchange comparison records row overlap,
   row-shape, and top numeric deltas for comparable surfaces.
5. `MEASURE-HP243-005`: prior HPHYS evidence review identifies next focus
   targets with evidence references.

## Deliverables
1. `artifacts/hphys0243-run-evidence.md`
2. `artifacts/hphys0243-hillslope-semantic-summary.md`
3. `artifacts/hphys0243-watershed-semantic-summary.md`
4. `artifacts/hphys0243-prior-hphys-review.md`
5. `artifacts/hphys0243-focus-recommendations.md`
6. `artifacts/gate-results.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/hphys0243_disposition.md`
9. `artifacts/worker-handoff.md`

## Mandatory Sequence
1. Verify available runfile, baseline, and comparator inputs.
2. Build current runner binaries.
3. Execute `H1..H39` hillslope rerun.
4. Execute watershed rerun from fresh pass shards.
5. Run semantic comparisons and aggregate summaries.
6. Review prior HPHYS evidence and publish recommendation/disposition.

## Autonomous Execution Intent
Execute this diagnostics package end-to-end through disposition without
requesting additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts explicitly label `Static:` and/or `Ran:`.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py`
- `/workdir/openWEPP/tools/legacy_comparison_suite/configs/pl14s_wat_tolerances.json`
- `/tmp/unpalatable_parity_20260529T192707Z/runs/`
- `/tmp/unpalatable_parity_20260529T192707Z/reports/hillslope/baseline_partitions/`
- `/wc1/runs/un/unpalatable-rind/wepp/output/interchange/`
- Prior HPHYS residual/disposition artifacts under `docs/work-packages/`.

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260602-hphys0243-post-0242-39-hillslope-watershed-parity-readjudication-001/**`

## Phase Plan
### Phase A - Setup
- Verify local input roots and create a fresh `/tmp/hphys0243_*` run root.

### Phase B - Rerun and Compare
- Run all hillslopes, run watershed, and generate semantic/interchange
  comparison artifacts.

### Phase C - Review and Disposition
- Review prior HPHYS evidence, publish focus recommendations, and disposition
  the diagnostic package.

## Exit Criteria
- Closure measures are either satisfied or explicit blockers are recorded with
  command evidence.
- Next focus recommendation is specific enough to scaffold or execute a
  follow-on remediation package.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local diagnostics/tests/docs only; no credentials/network writes.
