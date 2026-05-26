# 20260526-erod17-route-branch-contract-derived-tests-and-preimplementation-gate-001

## Status
- state: package-complete-with-hold
- date: 2026-05-26
- timezone: UTC
- decision: HOLD

## Objective
Execute EROD17 by implementing route-branch contract-derived test vectors for
EROD16 canonical authority (`mshear` branch families, deposition-end branch,
`ndep` follow-up, and `qostar` threshold behavior) and recording
pre-implementation gate evidence before route runtime migration packages.

## Why This Package Exists
ROUTEPLAN01 queued EROD17 as the contract-first step-2 package after EROD16
contract authority closure. Route runtime migration packages (EROD18/EROD19)
must not proceed without explicit test vectors and a recorded pre-migration
failure baseline.

## Scope
### Included
- Contract-derived route-branch vector implementation in:
  - `tests/integration/erod14_wave2_multiofe_enrichment_kernel_contract.rs`
- Pre-migration expected-failure baseline execution for ignored EROD17 vectors.
- Pre-implementation contract gate evidence for EROD18/EROD19 readiness.
- Governance, review, verification, and handoff artifacts.

### Explicitly Out of Scope
- Canonical contract authority amendments (EROD16 scope unless contradiction).
- Runtime state-topology or production route kernel edits (EROD18/EROD19 scope).
- Sediment-routing parity rerun and hold-lift disposition (EROD21 scope).

## Deliverables
1. `artifacts/erod17-contract-derived-test-matrix.md`
2. `artifacts/erod17-pre-migration-failure-baseline.md`
3. `artifacts/erod17-contract-implementation-evidence.md`
4. `artifacts/erod17-contract-test-implementation-evidence.md`
5. `artifacts/erod17-preimplementation-contract-gate.md`
6. `artifacts/erod17-implementation-and-test-evidence.md`
7. `artifacts/erod17-kernel-profile-compliance-checklist.md`
8. `artifacts/owned-file-manifest.md`
9. `artifacts/gate-results.md`
10. `artifacts/erod17_disposition.md`
11. `artifacts/worker-handoff.md`
12. `artifacts/review_agent_a.md`
13. `artifacts/review_agent_b.md`
14. `artifacts/verification_agent_a.md`
15. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
For downstream code-authoring packages:
1. implement canonical contract amendments,
2. implement contract-derived tests,
3. record pre-implementation contract gate evidence,
4. modify production code.

EROD17 executes steps 2 and 3 for route-branch migration scope.

## Autonomous Execution Intent (Required)
This package is execution-ready and complete for end-to-end autonomous
execution through disposition without additional user intervention unless
hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts include explicit `Static:` and/or `Ran:` labels.

## Provenance and Authority Posture
- Canonical authority remains in `SC-*` contracts.
- Legacy provenance baseline:
  `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- No heuristic/proxy process-physics substitutions are allowed.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260526-routeplan01-hillslope-sediment-routing-assessment-and-queue-001/artifacts/sediment-routing-wp-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260526-erod16-route-branch-contract-authority-and-routine-map-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260526-erod16-route-branch-contract-authority-and-routine-map-001/artifacts/erod16_disposition.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/tests/integration/erod14_wave2_multiofe_enrichment_kernel_contract.rs`

## Intended Write Set
- `tests/integration/erod14_wave2_multiofe_enrichment_kernel_contract.rs`
- `docs/work-packages/README.md`
- `docs/work-packages/20260526-erod17-route-branch-contract-derived-tests-and-preimplementation-gate-001/**`

## Phase Plan
### Phase A - Intake and test-scope freeze
- Confirm EROD16 authority closure and EROD17 vector requirements.

### Phase B - Contract-derived test authoring
- Implement EROD17 route-branch vectors as explicit pre-migration tests.

### Phase C - Pre-implementation contract gate
- Run targeted test commands, capture control run and expected-failure
  baseline from ignored vectors.

### Phase D - Governance and handoff
- Complete evidence, dual review, dual verification, and downstream handoff.

### Phase E - Disposition
- Keep package disposition in `HOLD` until route runtime migration packages
  close route-branch publication and algorithmic parity gaps.

## Exit Criteria
- EROD17 route-branch vectors exist in integration tests.
- Control run passes with EROD17 vectors ignored by default.
- Ignored-vector run fails with expected pre-migration route-branch gaps.
- Pre-implementation gate evidence is recorded explicitly.
- Required governance artifacts are complete with truthful labels.
- Non-doc file changed (`tests/integration/...`), so package records targeted
  test runs used for this test-authoring scope.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: test-authoring and governance only; no production runtime edits.

## Execution Outcome Summary
- Added five ignored EROD17 route-branch vectors.
- Default targeted integration suite passes while vectors remain ignored.
- Ignored-run fails as expected due missing route-branch publication symbols,
  confirming pre-migration gap posture for EROD18/EROD19.
