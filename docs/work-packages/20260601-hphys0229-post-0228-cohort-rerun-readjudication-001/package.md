# 20260601-hphys0229-post-0228-cohort-rerun-readjudication-001

## Status
- state: completed
- date: 2026-06-01
- timezone: America/Los_Angeles
- decision: hold

## Objective
Carry out immediate-next actions from HPHYS0228 by executing a fresh
`unpalatable-rind` 39-hillslope rerun/readjudication lane and publishing a
current residual-family delta baseline for `Dp`, `latqcc`, `Total-Soil`,
`SoilWaterTotal`, and `ProfileFCStore`.

## Why This Package Exists
HPHYS0227 introduced WB19 runtime changes and HPHYS0228 restored WB14
`ksatadj` guardrails, but the integrated residual-family cohort has not been
re-measured since HPHYS0224. Immediate next actions require a new readjudicated
baseline before selecting the next production remediation slice.

## Scope
### Included
- Build + execute `openwepp-cli-hill` for `H1..H39` using existing
  `unpalatable-rind` runfiles.
- Recompute semantic comparator reports against baseline partitions with valid
  settings (`--candidate-year-offset 2012`, no partition filter).
- Aggregate `hillslope_semantic_summary.{json,tsv}` and compute deltas versus
  HPHYS0224 summary.
- Re-run immediate guardrail suites:
  - WB14 `ksatadj` success-lane contract test,
  - required Level-4 WB19 constitutive suites.
- Publish disposition and next remediation handoff.

### Explicitly Out of Scope
- Production kernel/runtime code edits.
- Science-contract or registry amendments.
- Watershed reruns.

## Closure Measures (Required)
1. `MEASURE-HP229-001`: fresh 39-hillslope batch rerun artifacts are produced
   with `39/39` successful hillslope executions.
2. `MEASURE-HP229-002`: semantic comparator artifacts are produced with valid
   row alignment (`common_row_count > 0` for all hillslopes).
3. `MEASURE-HP229-003`: monitored-family deltas versus HPHYS0224 are published
   for `Dp`, `latqcc`, `Total-Soil`, `SoilWaterTotal`, `ProfileFCStore`.
4. `MEASURE-HP229-004`: WB14 guardrail and required Level-4 WB19 suites pass.
5. `MEASURE-HP229-005`: workspace gates pass (`fmt`, `clippy`, `test`, `deny`).
6. `MEASURE-HP229-006`: artifacts/disposition/handoff are updated with
   truthfulness labels and next remediation target recommendation.

## Deliverables
1. `artifacts/hphys0229-residual-authority-gap-matrix.md`
2. `artifacts/hphys0229-contract-implementation-evidence.md`
3. `artifacts/hphys0229-contract-test-implementation-evidence.md`
4. `artifacts/hphys0229-preimplementation-contract-gate.md`
5. `artifacts/hphys0229-implementation-and-test-evidence.md`
6. `artifacts/hphys0229-kernel-profile-compliance-checklist.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/hphys0229_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Sequence (Required)
1. Capture pre-implementation gate evidence (diagnostics-only scope).
2. Execute rerun + semantic comparison lane.
3. Execute guardrail tests and workspace gates.
4. Publish readjudication artifacts and disposition.

## Autonomous Execution Intent (Required)
Execute phases end-to-end through disposition without requesting additional
user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts explicitly label `Static:` and/or `Ran:`.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py`
- `/tmp/hphys0224_20260601T054337Z/parity/reports/hillslope_semantic_summary.json`
- `/tmp/unpalatable_parity_20260529T192707Z/reports/hillslope/baseline_partitions/`
- `/workdir/openWEPP/docs/work-packages/20260601-hphys0228-wb14-ksatadj-success-lane-restoration-001/artifacts/worker-handoff.md`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260601-hphys0229-post-0228-cohort-rerun-readjudication-001/**`

## Phase Plan
### Phase A - Intake and setup
- Freeze scope to HPHYS0228 immediate-next residual-family readjudication.
- Create run root and verify required baseline/runfile dependencies.

### Phase B - Rerun and semantic comparison
- Execute `H1..H39` runs and semantic comparisons.
- Aggregate summary artifacts.

### Phase C - Readjudication and closeout
- Compute deltas versus HPHYS0224 monitored families.
- Execute guardrail suites + workspace gates.
- Publish disposition and handoff.

## Exit Criteria
- `MEASURE-HP229-001..006` satisfied and evidenced.
- Integrated HPHYS stream remains explicit `HOLD` pending next production
  remediation package.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local diagnostics/tests/docs only; no credentials/network.
