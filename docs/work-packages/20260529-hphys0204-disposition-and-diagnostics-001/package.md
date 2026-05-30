# 20260529-hphys0204-disposition-and-diagnostics-001

## Status
- state: completed
- date: 2026-05-30
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Execute integrated closure diagnostics and disposition for HPHYS0202/0203 by
combining contract-authoritative physics evidence, workspace gates, and
semantic comparator diagnostics into a promotability decision.

## Why This Package Exists
After implementation packages close targeted process lineage and robustness
gates, we still need a formal disposition wave that interprets parity deltas
without allowing parity-only metrics to override process authority.

## Scope
### Included
- Re-run required validation gates and targeted cohorts.
- Generate integrated physics-evidence and parity-diagnostic summary.
- Classify residual deviations by confidence tier and likely root-cause family.
- Publish HOLD/GO disposition and immediate-next-action queue.

### Explicitly Out of Scope
- New kernel feature implementation unrelated to disposition findings.
- Watershed/channel migration scope outside hillslope follow-up.
- Release packaging/deployment.

## Closure Measures (Required)
1. `MEASURE-HP204-001`: full workspace gates pass and are logged
   (`fmt/clippy/test/deny`).
2. `MEASURE-HP204-002`: integrated diagnostics artifact includes:
   contract closure status, robustness-test status, and semantic comparator
   residual summary with confidence-tier labels.
3. `MEASURE-HP204-003`: final disposition explicitly states promotability based
   on process authority first, with parity deviations treated as diagnostic.
4. `MEASURE-HP204-004`: worker handoff defines scoped next packages for any
   unresolved residual families.

## Deliverables
1. `artifacts/hphys0204-physics-gap-matrix.md`
2. `artifacts/hphys0204-contract-implementation-evidence.md`
3. `artifacts/hphys0204-contract-test-implementation-evidence.md`
4. `artifacts/hphys0204-preimplementation-contract-gate.md`
5. `artifacts/hphys0204-implementation-and-test-evidence.md`
6. `artifacts/hphys0204-kernel-profile-compliance-checklist.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/hphys0204_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Confirm closure status of prior contract amendments/tests.
2. Execute validation gates and diagnostics with no production defaults.
3. Record pre-disposition gate evidence.
4. Publish disposition based on process authority and explicit residual triage.

## Autonomous Execution Intent (Required)
This package must execute end-to-end through disposition without requesting
additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:`.

## Provenance and Authority Posture
- Canonical process authority remains `SC-*` contracts.
- Legacy migration comparator anchor remains
  `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Parity residuals are interpreted as investigation signals unless linked to a
  contract-authoritative process defect.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260529-hphys0201-physics-first-gate-reframe-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260529-hphys0202-profile-fc-wp-lineage-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260529-hphys0203-physics-robustness-test-suite-001/package.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SOIL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260529-hphys0204-disposition-and-diagnostics-001/**`
- `docs/specifications/science-contracts/index.md` (if diagnostics classification
  references require registry updates)
- `tools/legacy_comparison_suite/**` (only if disposition diagnostics formatting
  updates are needed)

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm HPHYS0204 authorization and evidence sources from HPHYS0202/0203.

### Phase B - Evidence intake
- Ingest contract/test closure evidence from prior packages.

### Phase C - Validation gate execution
- Execute workspace validation commands and collect logs.

### Phase D - Cohort diagnostics
- Run targeted hillslope cohorts and comparator diagnostics.
- Classify deviations by confidence tier and family.

### Phase E - Disposition synthesis
- Produce integrated closure report and HOLD/GO decision with rationale.

### Phase F - Dual review, dual verification, disposition
- Complete review/verification artifacts and publish worker handoff.

## Exit Criteria
- Closure measures `MEASURE-HP204-001..004` are satisfied and evidenced.
- Disposition rationale is process-authority-first and reproducible.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: validation/disposition evidence updates only; no external
  auth/network surface changes.
