# 20260530-hphys0215-coupled-family-remediation-planning-001

## Status
- state: completed
- date: 2026-05-30
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Execute HPHYS0215 as the coupled-family remediation planning package after
HPHYS0214, producing an approved contract-first implementation queue for
`ProfileFCStore`, `Dp`, `latqcc`, `Total-Soil`, and `SoilWaterTotal`.

## Why This Package Exists
HPHYS0214 completed integrated readjudication with `HOLD` and explicitly queued
HPHYS0215 to convert residual blocker families into bounded, executable
remediation streams with objective closure criteria.

## Scope
### Included
- Ingest HPHYS0214 disposition and residual matrix evidence.
- Reconfirm required workspace gate health for planning credibility.
- Build family-by-family remediation decomposition:
  - authoritative contracts and symbol lineage touched,
  - bounded production write set,
  - required contract-first sequencing,
  - measurable closure targets.
- Publish the approved follow-on queue (`HPHYS0216+`) with explicit ownership.

### Explicitly Out of Scope
- Production kernel code changes.
- New semantic parity reruns beyond planning evidence needs.
- Watershed/channel migration work.

## Closure Measures (Required)
1. `MEASURE-HP215-001`: required workspace gates pass and are recorded.
2. `MEASURE-HP215-002`: each open family has a remediation stream with:
   contracts, ownership, intended write set, and closure criteria.
3. `MEASURE-HP215-003`: disposition preserves process-authority-first posture
   and does not claim hold-lift.
4. `MEASURE-HP215-004`: handoff queue is actionable (phase ordering, owners,
   objective evidence requirements).

## Deliverables
1. `artifacts/hphys0215-remediation-streams.md`
2. `artifacts/hphys0215-contract-implementation-evidence.md`
3. `artifacts/hphys0215-contract-test-implementation-evidence.md`
4. `artifacts/hphys0215-preimplementation-contract-gate.md`
5. `artifacts/hphys0215-implementation-and-test-evidence.md`
6. `artifacts/hphys0215-kernel-profile-compliance-checklist.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/hphys0215_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Ingest canonical contract authority and HPHYS0214 residual evidence.
2. Ingest contract-derived test obligations for each remediation stream.
3. Record pre-implementation contract gate for follow-on package queue.
4. Publish remediation queue/disposition/handoff.

## Autonomous Execution Intent (Required)
This package must execute end-to-end through disposition without requesting
additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:`.

## Provenance and Authority Posture
- Canonical authority lives in
  `docs/specifications/science-contracts/contracts/SC-*.md`.
- Legacy baseline comparator provenance remains
  `/workdir/wepp-forest_260430_baseline` at commit
  `dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Comparator deltas are investigation signals; process-authoritative closure is
  the promotability gate.

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
- `/workdir/openWEPP/docs/work-packages/20260530-hphys0214-integrated-hold-lift-readjudication-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260530-hphys0214-integrated-hold-lift-readjudication-001/artifacts/hphys0214_disposition.md`
- `/workdir/openWEPP/docs/work-packages/20260530-hphys0214-integrated-hold-lift-readjudication-001/artifacts/hphys0214-residual-gap-matrix.md`
- `/workdir/openWEPP/docs/work-packages/20260530-hphys0214-integrated-hold-lift-readjudication-001/artifacts/worker-handoff.md`
- `/tmp/hphys0214_20260531T004200Z/diagnostics/hphys0214_integrated_family_summary.json`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260530-hphys0215-coupled-family-remediation-planning-001/**`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm HPHYS0215 authorization from HPHYS0214 handoff and freeze scope to
  coupled-family remediation planning only.

### Phase B - Authority and evidence intake
- Ingest canonical contract authority and HPHYS0214 residual evidence.

### Phase C - Remediation stream decomposition
- Publish bounded implementation streams and closure measures for HPHYS0216+.

### Phase D - Validation gates
- Execute required workspace gates and record results.

### Phase E - Disposition and queue handoff
- Publish package disposition and executable next-package queue.

## Exit Criteria
- Closure measures `MEASURE-HP215-001..004` are satisfied and evidenced.
- All five blocker families have bounded remediation streams with explicit
  ownership and objective closure criteria.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: planning/evidence package only; no production behavior changes.
