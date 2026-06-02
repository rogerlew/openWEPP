# 20260602-hphys0244-h1-h7-h39-storage-wb18-lineage-diagnostics-001

## Status
- state: completed
- date: 2026-06-02
- timezone: America/Los_Angeles
- decision: HOLD_PENDING_WB11_WB18_LAYER_TELEMETRY_AND_MUTABLE_STORAGE_CLOSURE

## Objective
Execute the first focused post-HPHYS0243 diagnostic work-package for `H1`,
`H7`, and `H39`, assessing layer state availability (`st`/`theta`),
`Total-Soil`, `SoilWaterTotal`, and WB18 `Dp`/`Pe` lineage evidence before
implementation scoping.

## Why This Package Exists
HPHYS0243 showed runoff closure but persistent large storage/percolation
residuals. The next useful step is not another broad cohort rerun; it is a
small, inspectable slice that compares representative hillslopes across the
wet-to-dry residual range and determines whether current artifacts expose the
layer state needed to isolate WB18/WB11 lifecycle defects.

## Scope
### Included
- Use the fresh HPHYS0243 `unpalatable-rind` candidate outputs as the
  post-HPHYS0242 candidate evidence base.
- Compare baseline and candidate `H.wat` outputs for `H1`, `H7`, and `H39`.
- Audit available emitted surfaces for per-layer `st`/`theta` and WB18 `Pe`.
- Inspect prior HPHYS root-cause evidence for WB11/WB18 storage lifecycle,
  `Dp`, `Total-Soil`, and `SoilWaterTotal` lineage.
- Publish a focus recommendation for the next implementation package.

### Explicitly Out of Scope
- Production code edits.
- Science-contract amendments.
- New runtime instrumentation.
- New process-physics implementation.
- Commit/push unless separately requested.

## Closure Measures
1. `MEASURE-HP244-001`: `H1`, `H7`, and `H39` baseline/candidate WAT
   comparison tables are generated for `Dp`, `Total-Soil`, and
   `SoilWaterTotal`.
2. `MEASURE-HP244-002`: layer `st`/`theta` and WB18 `Pe` availability is
   audited from current emitted artifacts.
3. `MEASURE-HP244-003`: prior HPHYS root-cause artifacts are reviewed for
   WB11/WB18 lifecycle and storage-publication lineage.
4. `MEASURE-HP244-004`: implementation focus recommendations are evidence
   backed and explicitly distinguish observed runtime output evidence from
   static lineage inference.

## Deliverables
1. `artifacts/hphys0244-data-availability-audit.md`
2. `artifacts/hphys0244-h1-h7-h39-storage-dp-summary.md`
3. `artifacts/hphys0244-layer-state-lineage-probe.md`
4. `artifacts/hphys0244-prior-root-cause-review.md`
5. `artifacts/hphys0244-focus-recommendations.md`
6. `artifacts/gate-results.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/hphys0244_disposition.md`
9. `artifacts/worker-handoff.md`

## Mandatory Sequence
1. Verify HPHYS0243 candidate outputs and baseline comparator roots.
2. Audit emitted artifact schemas for layer state and WB18 `Pe` availability.
3. Generate targeted `H1`/`H7`/`H39` storage and `Dp` diagnostic summaries.
4. Review prior HPHYS root-cause artifacts for WB11/WB18 lifecycle linkage.
5. Publish recommendations and package disposition.

## Autonomous Execution Intent
Execute this diagnostics package end-to-end through disposition without
requesting additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts explicitly label `Static:` and/or `Ran:`.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260602-hphys0243-post-0242-39-hillslope-watershed-parity-readjudication-001/`
- `/tmp/hphys0243_20260602T042747Z/parity/`
- `/tmp/unpalatable_parity_20260529T192707Z/reports/hillslope/baseline_partitions/`
- `/wc1/runs/un/unpalatable-rind/wepp/output/interchange/`
- Prior HPHYS residual/disposition artifacts under `docs/work-packages/`.

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260602-hphys0244-h1-h7-h39-storage-wb18-lineage-diagnostics-001/**`

## Phase Plan
### Phase A - Setup
- Verify local evidence roots and scaffold package artifacts.

### Phase B - Targeted Diagnostics
- Generate `H1`, `H7`, and `H39` storage/percolation diagnostic summaries and
  artifact-surface availability evidence.

### Phase C - Review and Disposition
- Review prior root-cause evidence, publish next-focus recommendations, and
  disposition the diagnostic package.

## Exit Criteria
- Closure measures are either satisfied or explicit blockers are recorded with
  command evidence.
- The next remediation target is specific enough to scaffold a code-authoring
  package if authorized.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local diagnostics/docs only; no credentials/network writes.
