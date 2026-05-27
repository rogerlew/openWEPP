# 20260527-wshedimpl07-watershed-impoundment-continuity-migration-001

## Status
- state: package-complete-with-hold
- date: 2026-05-27
- timezone: UTC
- decision: HOLD

## Objective
Execute WSHED07 by migrating WS12 impoundment continuity behavior into
production watershed execution, including RK4 stage integration, adaptive
timestep retry, and regime-transition retry controls, then promote the matching
WSHED03 expected-failure vector to active conformance.

## Why This Package Exists
WSHEDIMPL04 closed parser-to-runtime impoundment coefficient seam requirements
for inactive-structure conformance lanes. WSHEDIMPL06 left impoundment
continuity migration (`GAP-IMPOUND-005`) as the primary WS12 blocker before
watershed parquet activation (WSHED08) and final hold-lift validation
(WSHED09).

## Scope
### Included
- WS12 impoundment continuity runtime migration in
  `crates/openwepp-watershed-orchestrator/src/lib.rs`:
  - RK4 stage integration,
  - adaptive timestep retry controller,
  - regime-transition retry controls,
  - duration-capped routing horizon for deterministic timestep stability.
- Promotion of
  `wshed03_contract_ws12_vector_requires_regime_transition_timestep_stability`
  from ignored expected-failure to active conformance.
- Canonical contract and index synchronization for WS12 closure posture:
  `SC-IMPOUND-001`, `SC-SYSTEM-001`, `science-contracts/index.md`.
- Package evidence, gate, review, verification, disposition, and handoff
  updates.

### Explicitly Out of Scope
- Active-structure branch payload projection expansion for impoundment
  coefficients (remains fail-closed under `GAP-IMPOUND-006` /
  `GAP-SYSTEM-007`).
- Full watershed channel sediment process migration (`chnero/chnrt/detach`).
- Watershed parquet writer activation (WSHED08) and end-to-end hold-lift
  disposition (WSHED09).

## Deliverables
1. `artifacts/wshedimpl07-impoundment-continuity-migration-report.md`
2. `artifacts/wshedimpl07-contract-implementation-evidence.md`
3. `artifacts/wshedimpl07-contract-test-implementation-evidence.md`
4. `artifacts/wshedimpl07-preimplementation-contract-gate.md`
5. `artifacts/wshedimpl07-implementation-and-test-evidence.md`
6. `artifacts/wshedimpl07-kernel-profile-compliance-checklist.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/wshedimpl07_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
Kernel-affecting sequencing remains mandatory:
1. implement required canonical contract amendments,
2. implement contract-derived tests,
3. record pre-implementation contract gate evidence, then
4. modify production code.

WSHEDIMPL07 includes step 1 (contract gap and authority synchronization), step
2 (WS12 vector promotion), step 3 (gate evidence), and step 4 (runtime
migration).

## Autonomous Execution Intent (Required)
This package is execution-ready and must proceed end-to-end through disposition
without requesting additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must include explicit `Static:` and/or `Ran:` sections.
Do not claim command execution unless it was actually run.

## Provenance and Authority Posture
- Canonical authority is in
  `docs/specifications/science-contracts/contracts/SC-*.md`.
- Work-package artifacts are evidence and do not replace canonical authority.
- Legacy migration provenance defaults to
  `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- No heuristic/proxy process-physics substitutions are allowed in production
  migration closure claims.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedplan01-watershed-channel-routing-orchestration-parquet-assessment-001/artifacts/watershed-channel-routing-orchestration-parquet-wp-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl02-watershed-contract-derived-tests-and-preimplementation-gate-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl02-watershed-contract-derived-tests-and-preimplementation-gate-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl04-watershed-runtime-seam-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl06-watershed-channel-sediment-routing-foundation-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/crates/openwepp-watershed-orchestrator/src/lib.rs`
- `/workdir/openWEPP/tests/integration/ws12_impoundment_physics_equivalence_contract.rs`
- `/workdir/wepp-forest_260430_baseline/src/imphnw.for`
- `/workdir/wepp-forest_260430_baseline/src/impflo.for`
- `/workdir/wepp-forest_260430_baseline/src/impmai.for`
- `/workdir/wepp-forest_260430_baseline/src/wshiqi.for`
- `/workdir/wepp-forest_260430_baseline/src/wshimp.for`

## Intended Write Set
- `docs/work-packages/20260527-wshedimpl07-watershed-impoundment-continuity-migration-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-watershed-orchestrator/src/lib.rs`
- `tests/integration/ws12_impoundment_physics_equivalence_contract.rs`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm queue authority and WSHED03 expected-failure baseline for WS12
  impoundment timestep-stability coverage.

### Phase B - Contract and contract-test updates
- Amend canonical WS12 gap/status posture in `SC-IMPOUND-001` and
  `SC-SYSTEM-001`.
- Promote WSHED03 WS12 timestep-stability vector from ignored expected-failure
  to active conformance.

### Phase C - Runtime implementation
- Implement RK4 + adaptive/retry continuity execution in impoundment production
  kernel path with preserved typed guard IDs.

### Phase D - Validation and governance evidence
- Run scoped watershed integration tests and required repository gates.
- Update kernel-profile checklist, evidence artifacts, dual review, and dual
  verification artifacts.

### Phase E - Disposition and handoff
- Record WSHED07 closure posture and route residual blockers to WSHED08/09.

## Exit Criteria
- WSHED03 WS12 timestep-stability vector is active (not ignored) and passing.
- WS12 runtime uses continuity integration path with adaptive/retry controls and
  retained typed guard continuity (`WKERNEL-WS10-IMPOUNDMENT-E-001..003`).
- `GAP-IMPOUND-005` is dispositioned in canonical authority.
- Required artifacts are complete with truthful evidence labeling.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local kernel behavior + tests only; no network/credential scope.

## Execution Outcome Summary
- WSHED07 scoped objective is complete:
  - production impoundment execution now routes WS12 continuity using RK4 stage
    integration with adaptive timestep retry and regime-transition retry logic;
  - routing horizon is duration-capped for deterministic timestep behavior;
  - WSHED03 WS12 timestep-stability vector is active and passing.
- Canonical contract posture is synchronized:
  - `SC-IMPOUND-001` dispositioned `GAP-IMPOUND-005` to `closed`;
  - `SC-SYSTEM-001` normalized `GAP-SYSTEM-007` impact language to retain only
    active-structure projection blockers;
  - `science-contracts/index.md` notes were updated for WS12 closure posture.
- Program-level watershed closure remains `HOLD` pending WSHED08/09 and
  unresolved active-structure coefficient projection expansion.
