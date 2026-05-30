# 20260530-hphys0211-coupled-threshold-root-cause-ledger-001

## Status
- state: completed
- date: 2026-05-30
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Execute HPHYS0211 as the coupled-threshold residual root-cause decomposition
package after HPHYS0210, producing a concrete symbol-path defect ledger for:
`ProfileFCStore`, `Dp`, `latqcc`, `Total-Soil`, and `SoilWaterTotal`.

## Why This Package Exists
HPHYS0210 disposition held the lane and queued HPHYS0211 to decompose ownership
of the coupled residual families before remediation packages (HPHYS0212+). This
package closes that decomposition step with executable evidence and a bounded
handoff queue.

## Scope
### Included
- Ingest authority and residual evidence from HPHYS0208, HPHYS0209, HPHYS0210.
- Build a rooted lineage ledger from contract authority -> runtime symbols ->
  production implementation points -> residual signatures.
- Re-run required workspace validation gates and targeted contract tests.
- Publish disposition and immediate-next-package remediation queue.

### Explicitly Out of Scope
- Production kernel/runtime behavior edits.
- New contract amendments.
- Watershed/channel/impoundment work.

## Closure Measures (Required)
1. `MEASURE-HP211-001`: required workspace gates pass and are recorded.
2. `MEASURE-HP211-002`: residual matrix includes per-family symbol-path
   ownership and concrete root-cause hypotheses backed by code/diagnostic
   evidence.
3. `MEASURE-HP211-003`: disposition keeps process-authority-first `HOLD`/`GO`
   posture and names executable follow-on packages when `HOLD` remains.
4. `MEASURE-HP211-004`: contract-first sequence is honored (authority/test
   intake before any implementation claims).

## Deliverables
1. `artifacts/hphys0211-residual-gap-matrix.md`
2. `artifacts/hphys0211-contract-implementation-evidence.md`
3. `artifacts/hphys0211-contract-test-implementation-evidence.md`
4. `artifacts/hphys0211-preimplementation-contract-gate.md`
5. `artifacts/hphys0211-implementation-and-test-evidence.md`
6. `artifacts/hphys0211-kernel-profile-compliance-checklist.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/hphys0211_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Ingest canonical contract authority and upstream closure evidence.
2. Ingest contract-derived tests and rerun targeted vectors.
3. Record pre-implementation contract gate for HPHYS0211 decomposition scope.
4. Publish root-cause ledger/disposition/handoff.

## Autonomous Execution Intent (Required)
This package executes end-to-end through disposition without requesting user
direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:`.

## Provenance and Authority Posture
- Canonical authority lives in
  `docs/specifications/science-contracts/contracts/SC-*.md`.
- Legacy comparator baseline provenance remains
  `/workdir/wepp-forest_260430_baseline` commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Comparator residuals are diagnostic signals; process-authoritative closure is
  the promotability gate.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260530-hphys0208-fc-threshold-coupled-residual-closure-001/artifacts/hphys0208_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260530-hphys0209-profilewp-near-closed-adjudication-001/artifacts/hphys0209_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260530-hphys0210-integrated-hold-lift-adjudication-001/artifacts/hphys0210_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260530-hphys0210-integrated-hold-lift-adjudication-001/artifacts/worker-handoff.md`
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
- `/tmp/hphys0208_20260530T155837Z/parity/reports/hillslope_semantic_summary.json`
- `/tmp/hphys0208_20260530T155837Z/parity/reports/semantic/H*.semantic.json`
- `/tmp/hphys0209_20260530T171007Z/parity/reports/hphys0209_profilewp_focus_summary.json`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260530-hphys0211-coupled-threshold-root-cause-ledger-001/**`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm HPHYS0211 authorization from HPHYS0210 handoff and freeze scope to
  coupled-threshold residual root-cause decomposition.

### Phase B - Contract and evidence intake
- Ingest canonical contract authority and upstream package evidence.

### Phase C - Root-cause decomposition
- Build per-family defect ledger with concrete symbol-path ownership and
  remediation targets.

### Phase D - Validation gates
- Execute required workspace gates and targeted contract-derived tests.

### Phase E - Disposition and handoff
- Publish `HOLD`/`GO` decision and immediate next-package queue.

## Exit Criteria
- Closure measures `MEASURE-HP211-001..004` are satisfied and evidenced.
- Root-cause ownership for all open families is explicit and actionable.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: documentation/evidence package execution only; no network/auth
  surface changes.
