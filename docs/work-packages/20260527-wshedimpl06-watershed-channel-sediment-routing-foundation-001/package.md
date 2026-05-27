# 20260527-wshedimpl06-watershed-channel-sediment-routing-foundation-001

## Status
- state: package-complete-with-hold
- date: 2026-05-27
- timezone: UTC
- decision: HOLD

## Objective
Execute WSHED06 by implementing WS11 channel sediment publication-family
closure (`ws10_channel_{id}_qsed`, `ws10_channel_{id}_tc`) with typed
fail-closed guards, promoting the matching WSHED03 expected-failure vector,
and narrowing remaining `chnero/chnrt/detach` migration blockers.

## Why This Package Exists
WSHEDIMPL05 closed WS11 `ipeak > 2` wave-state publication and left channel
sediment runtime closure as the next watershed blocker. WSHED03 already
contains an expected-failure WS11 sediment vector requiring production
publication of `qsed` and `tc` symbols.

## Scope
### Included
- Channel sediment publication-family implementation in watershed production
  channel execution:
  - `ws10_channel_{id}_qsed`
  - `ws10_channel_{id}_tc`
- Deterministic contributor-sediment carry-through for channel output-state
  publication with typed fail-closed non-finite/domain guards.
- Promotion of
  `wshed03_contract_channel_sediment_vector_requires_channel_sediment_publication_family`
  from ignored expected-failure to active conformance.
- Contract gap-status synchronization for routing/system sediment residuals.
- Package evidence, gate, review, verification, disposition, and handoff
  updates.

### Explicitly Out of Scope
- Full baseline-authoritative channel sediment process migration of complete
  `chnero/chnrt/detach` routine families (segment-level shear, detachment,
  deposition, transport-capacity iteration).
- WS12 RK4/adaptive impoundment regime-transition migration (WSHED07 scope).
- Watershed parquet writer activation (WSHED08 scope).

## Deliverables
1. `artifacts/wshedimpl06-channel-sediment-routing-report.md`
2. `artifacts/wshedimpl06-contract-implementation-evidence.md`
3. `artifacts/wshedimpl06-contract-test-implementation-evidence.md`
4. `artifacts/wshedimpl06-preimplementation-contract-gate.md`
5. `artifacts/wshedimpl06-implementation-and-test-evidence.md`
6. `artifacts/wshedimpl06-kernel-profile-compliance-checklist.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/wshedimpl06_disposition.md`
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

WSHEDIMPL06 executes steps 2, 3, and 4 for sediment publication-family closure;
step 1 is limited to minimal gap-row/disposition updates if closure evidence
warrants it.

## Autonomous Execution Intent (Required)
This package is execution-ready and must proceed end-to-end through
disposition without requesting additional user direction unless hard-blocked.

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
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedplan01-watershed-channel-routing-orchestration-parquet-assessment-001/artifacts/watershed-channel-routing-orchestration-parquet-wp-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl02-watershed-contract-derived-tests-and-preimplementation-gate-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl02-watershed-contract-derived-tests-and-preimplementation-gate-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl05-watershed-wave-routing-state-family-migration-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl05-watershed-wave-routing-state-family-migration-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/crates/openwepp-watershed-orchestrator/src/lib.rs`
- `/workdir/openWEPP/tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`
- `/workdir/wepp-forest_260430_baseline/src/chnero.for`
- `/workdir/wepp-forest_260430_baseline/src/chnrt.for`
- `/workdir/wepp-forest_260430_baseline/src/detach.for`

## Intended Write Set
- `docs/work-packages/20260527-wshedimpl06-watershed-channel-sediment-routing-foundation-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-watershed-orchestrator/src/lib.rs`
- `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm queue authority and WSHED03 expected-failure baseline for WS11
  channel sediment publication symbols.

### Phase B - Contract-derived test and gate preparation
- Promote WSHED03 WS11 channel sediment publication-family vector from ignored
  expected failure to active conformance.
- Record pre-implementation gate posture.

### Phase C - Runtime implementation
- Implement channel sediment publication symbols in production WS10 channel
  writeback with typed guard continuity.

### Phase D - Validation and governance evidence
- Run scoped integration tests and required gates.
- Update kernel-profile checklist, evidence artifacts, dual review, and dual
  verification artifacts.

### Phase E - Disposition and handoff
- Record WSHED06 closure posture for this publication-family scope and route
  remaining `chnero/chnrt/detach` parity blockers to follow-on work.

## Exit Criteria
- WS11 channel publication family emits `ws10_channel_{id}_qsed` and
  `ws10_channel_{id}_tc`.
- WSHED03 WS11 channel sediment vector is active (not ignored) and passing.
- Typed guard continuity is retained (fail-closed on invalid runtime domains).
- Required artifacts are complete with truthful evidence labeling.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local kernel behavior + tests only; no network/credential scope.

## Execution Outcome Summary
- WSHED06 scoped objective is complete:
  - WS11 channel outputs now publish sediment publication-family symbols on WS10
    channel nodes (`ws10_channel_{id}_qsed`, `ws10_channel_{id}_tc`) with
    fail-closed typed guard continuity.
  - WSHED03 sediment publication vector is promoted from ignored to active and
    passes.
- Canonical contract posture is synchronized:
  - `SC-ROUTE-001` records WSHED06 publication-family closure and narrows
    `GAP-ROUTE-009` to remaining full `chnero/chnrt/detach` process-parity
    migration.
  - `SC-SED-001` and `SC-SYSTEM-001` were updated to reflect that WSHED06
    closed publication-family scope while full watershed channel sediment
    process migration remains open.
- Program-level watershed closure remains `HOLD` pending WSHED07/08/09 and
  completion of full channel sediment process parity migration.
