# 20260531-hphys0217-post-0216d-coupled-family-rerun-readjudication-001

## Status
- state: completed
- date: 2026-05-31
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Execute HPHYS0217 post-HPHYS0216D rerun/readjudication by running a fresh
`unpalatable-rind` 39-hillslope semantic lane and re-evaluating coupled
residual families (`Dp`, `latqcc`, `Total-Soil`, `SoilWaterTotal`) with
`ProfileFCStore` treated as control after the FC layer+tail fix.

## Why This Package Exists
HPHYS0216D closed FC publication authority implementation and left integrated
hold-lift contingent on fresh rerun evidence for remaining coupled families.
This package executes that mandated rerun/adjudication step and publishes the
next implementation queue based on current evidence.

## Scope
### Included
- Fresh 39-hillslope rerun for `unpalatable-rind` using current workspace
  binaries and existing openWEPP runfiles.
- Semantic comparator execution per hillslope against existing baseline
  partition artifacts.
- Consolidated residual-family summary for `Dp`, `latqcc`, `Total-Soil`,
  `SoilWaterTotal`, and control `ProfileFCStore`.
- Integrated disposition (`HOLD`/`GO`) with explicit next-package queue.

### Explicitly Out of Scope
- Production kernel/runtime code edits.
- Canonical contract amendments.
- Watershed lane remediation.

## Closure Measures (Required)
1. `MEASURE-HP217-001`: all 39 hillslope reruns and semantic comparisons
   complete with `rc=0` status evidence.
2. `MEASURE-HP217-002`: package publishes machine-derived family summary
   (`fail_hillslopes`, `mean_abs_diff_avg`) and comparison to HPHYS0216
   reference summary.
3. `MEASURE-HP217-003`: package publishes explicit integrated `HOLD`/`GO`
   decision with traceable rationale for each monitored family.
4. `MEASURE-HP217-004`: worker handoff defines executable next implementation
   queue if any monitored family remains open.

## Deliverables
1. `artifacts/hphys0217-residual-gap-matrix.md`
2. `artifacts/hphys0217-contract-implementation-evidence.md`
3. `artifacts/hphys0217-contract-test-implementation-evidence.md`
4. `artifacts/hphys0217-preimplementation-contract-gate.md`
5. `artifacts/hphys0217-implementation-and-test-evidence.md`
6. `artifacts/hphys0217-kernel-profile-compliance-checklist.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/hphys0217_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Sequencing (Required)
1. Record no-code/no-contract preimplementation gate and scope freeze.
2. Execute fresh rerun + semantic comparison evidence capture.
3. Publish residual-family diagnostics and integrated disposition.
4. Publish next-package handoff with explicit write-set boundaries.

## Autonomous Execution Intent (Required)
This package executes end-to-end through disposition without requesting
additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:`.

## Provenance and Authority Posture
- Canonical authority remains in
  `docs/specifications/science-contracts/contracts/SC-*.md`.
- Baseline comparator provenance remains
  `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- This package performs diagnostics/readjudication only and does not alter
  physics authority text.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260531-hphys0216d-profilefc-normalized-tail-authority-reconciliation-001/artifacts/worker-handoff.md`
- `/tmp/unpalatable_parity_20260529T192707Z/reports/hillslope/baseline_partitions/`
- `/tmp/hphys0216_20260531T053959Z/parity/reports/hillslope_semantic_summary.json`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260531-hphys0217-post-0216d-coupled-family-rerun-readjudication-001/**`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm HPHYS0216D immediate-next-actions authorization.
- Freeze HPHYS0217 to rerun/readjudication only.

### Phase B - Rerun execution
- Execute fresh 39-hillslope openWEPP lane using existing runfiles.
- Capture per-hillslope run return codes.

### Phase C - Semantic diagnostics
- Execute semantic comparator for each hillslope.
- Aggregate monitored-family summary and compare against HPHYS0216 reference.

### Phase D - Disposition and handoff
- Publish residual gap matrix and integrated `HOLD`/`GO` disposition.
- Publish explicit follow-on implementation queue for unresolved families.

## Exit Criteria
- Closure measures `MEASURE-HP217-001..004` are satisfied and evidenced.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: diagnostics/documentation updates only; no interface or auth changes.
