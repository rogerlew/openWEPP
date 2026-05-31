# 20260531-hphys0220-wb19-coupled-flux-partition-diagnostics-001

## Status
- state: completed
- date: 2026-05-31
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Execute HPHYS0220 to diagnose WB19 coupled residual tradeoffs observed after
HPHYS0219 (`Dp` improvement with `latqcc`/total-soil regression), identify the
authoritative legacy process-physics surfaces missing from current openWEPP
WB19 implementation, and publish a contract-first remediation package handoff.

## Why This Package Exists
HPHYS0219 corrected WB19 `drfc` coefficient-family authority (`cpm -> coca`)
and improved `Dp` mean residuals, but simultaneously regressed `latqcc`,
`Total-Soil`, and `SoilWaterTotal` means, with fail saturation unchanged.
Observed directionality is deterministic across all 39 hillslopes, indicating
structural flux-partition coupling that needs explicit remediation sequencing.

## Scope
### Included
- Analyze HPHYS0218 vs HPHYS0219 semantic outputs on `unpalatable-rind`
  39-hillslope cohort.
- Quantify per-hillslope directional coupling across `Dp`, `latqcc`,
  `Total-Soil`, and `SoilWaterTotal`.
- Perform static baseline/openWEPP source-lineage audit for WB19 lateral/drain
  physics surfaces.
- Publish next remediation package scope with explicit contract/test/code gates.

### Explicitly Out of Scope
- Production kernel/runtime code edits.
- Canonical contract amendments.
- Fresh full rerun execution (HPHYS0219 rerun evidence is consumed).

## Closure Measures (Required)
1. `MEASURE-HP220-001`: publish reproducible cross-package delta evidence for
   HPHYS0218 vs HPHYS0219 coupled families.
2. `MEASURE-HP220-002`: publish source-level diagnosis of missing/partial WB19
   process-physics lineage in openWEPP relative to baseline authority.
3. `MEASURE-HP220-003`: publish explicit follow-on remediation package scope
   with contract-first sequencing and closure gates.
4. `MEASURE-HP220-004`: update work-package registry and handoff for immediate
   next execution.

## Deliverables
1. `artifacts/hphys0220-contract-implementation-evidence.md`
2. `artifacts/hphys0220-contract-test-implementation-evidence.md`
3. `artifacts/hphys0220-preimplementation-contract-gate.md`
4. `artifacts/hphys0220-implementation-and-test-evidence.md`
5. `artifacts/hphys0220-kernel-profile-compliance-checklist.md`
6. `artifacts/hphys0220-residual-gap-matrix.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/hphys0220_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Sequencing (Required)
1. Consume existing HPHYS0219 semantic evidence.
2. Perform static baseline/openWEPP WB19 source-lineage audit.
3. Record no-code preimplementation gate + compliance evidence.
4. Publish disposition and remediation handoff.

## Autonomous Execution Intent (Required)
This package executes end-to-end through disposition without requesting
additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts explicitly label `Static:` and/or `Ran:`.

## Provenance and Authority Posture
- Canonical authority remains in `docs/specifications/science-contracts/contracts/SC-*.md`.
- Baseline comparator/migration provenance:
  `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- This package diagnoses coupled behavior and prepares authoritative
  remediation; it does not alter production physics.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-WATBAL-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SUBHYD-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-PERC-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260531-hphys0219-wb19-coca-threshold-authority-correction-001/artifacts/worker-handoff.md`
- `/tmp/hphys0218_20260531T075251Z/parity/reports/hillslope_semantic_summary.json`
- `/tmp/hphys0219_20260531T083756Z/parity/reports/hillslope_semantic_summary.json`
- `/workdir/wepp-forest_260430_baseline/src/watbal.for`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260531-hphys0220-wb19-coupled-flux-partition-diagnostics-001/**`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm HPHYS0219 handoff authorization and freeze to diagnostics/remediation
  planning only.

### Phase B - Coupled delta analysis
- Quantify directionality of HPHYS0219 vs HPHYS0218 residual changes.

### Phase C - Source-lineage diagnosis
- Identify baseline WB19 process surfaces not yet represented in openWEPP
  lateral/drain kernels.

### Phase D - Disposition and follow-on queue
- Publish HOLD/GO decision and explicit follow-on remediation package handoff.

## Exit Criteria
- Closure measures `MEASURE-HP220-001..004` are satisfied and evidenced.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: diagnostics/documentation-only package.
