# 20260531-auth04-release-gate-authority-stack-integration-001

## Status
- state: queued
- date: 2026-05-31
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Integrate the correctness authority stack into release/CI workflows by wiring
constitutive suite classes into required, periodic, and manual gate lanes with
explicit fail semantics.

## Why This Package Exists
AUTH01-AUTH03 establish model, structure, and first executable suites. AUTH04
must enforce them operationally so disposition is workflow-backed.

## Scope
### Included
- Wire required constitutive suites into release-gates CI path.
- Add periodic/manual lanes for heavier suite classes with explicit triggers.
- Enforce failure-class policy (`hard-fail` vs `investigation`) in scripts and
  workflow outputs.
- Update runbooks and release procedure docs to reflect authority-stack gates.

### Explicitly Out of Scope
- New constitutive physics implementation.
- Non-authority CI refactors unrelated to gate integration.
- Comparator threshold policy rewrites beyond authority model alignment.

## Deliverables
1. `artifacts/contract-implementation-evidence.md`
2. `artifacts/contract-test-implementation-evidence.md`
3. `artifacts/preimplementation-contract-gate.md`
4. `artifacts/implementation-and-test-evidence.md`
5. `artifacts/kernel-profile-compliance-checklist.md`
6. `artifacts/owned-file-manifest.md`
7. `artifacts/gate-results.md`
8. `artifacts/disposition.md`
9. `artifacts/worker-handoff.md`
10. `artifacts/review_agent_a.md`
11. `artifacts/review_agent_b.md`
12. `artifacts/verification_agent_a.md`
13. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Amend contract/governance docs for workflow gate obligations.
2. Add contract-derived workflow tests/checks for lane selection and fail class.
3. Record pre-implementation contract gate evidence.
4. Implement release/workflow wiring and publish operational evidence.

## Autonomous Execution Intent (Required)
This package must execute end-to-end through disposition without requesting
additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:`.

## Provenance and Authority Posture
- Canonical process authority remains in `SC-*` contracts.
- External constitutive suite authority must cite primary references with
  explicit version/commit provenance.
- Legacy comparator parity remains investigation signal, not acceptance oracle.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/governance/correctness-reanchoring-keep-condemn-map.md`
- `/workdir/openWEPP/docs/governance/openwepp-release-procedure-draft.md`
- `/workdir/openWEPP/docs/work-packages/20260531-auth03-level4-constitutive-gate-bootstrap-001/package.md`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260531-auth04-release-gate-authority-stack-integration-001/**`
- `docs/specifications/science-contracts/index.md`
- `docs/specifications/correctness-authority-model.md`
- `docs/governance/openwepp-release-procedure-draft.md`
- `.github/workflows/release-gates.yml`
- `tools/release/run_release_candidate_gates.sh`
- `tools/release/README.md`

## Phase Plan
### Phase A - Scope freeze and AUTH03 intake
- Confirm queue authorization and freeze AUTH04 boundaries.

### Phase B - Contract/workflow authority amendments
- Define lane/failure semantics in canonical docs and release governance text.

### Phase C - Contract-derived workflow checks
- Add checks/tests that validate lane routing and non-blocking investigation
  behavior.

### Phase D - CI and release-gate integration
- Wire required suites into push/PR gates and periodic/manual suites into
  dispatch lanes.

### Phase E - Validation and disposition
- Run workflow lint/test gates and publish disposition with operational runbook.

## Exit Criteria
- Release workflow enforces required authority stack gates.
- Periodic/manual suites are routable and documented.
- Failure classes are machine-enforced and operator-visible.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: documentation/governance updates only; no runtime/network/auth
  surface changes.
