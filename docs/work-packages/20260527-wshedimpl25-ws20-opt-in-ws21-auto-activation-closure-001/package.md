# 20260527-wshedimpl25-ws20-opt-in-ws21-auto-activation-closure-001

## Status
- state: complete
- date: 2026-05-27
- timezone: UTC
- decision: HOLD

## Objective
Execute WSHEDIMPL25 immediate-next-action scope from WSHEDIMPL24 by closing the
residual WS20 opt-in unresolved-detachment fallback lane: make WS20 case12
opt-in execution auto-activate WS21 case34/detach continuation behavior, enforce
fail-closed `crfrac` requirements for WS20-only opt-in lanes, and preserve typed
guard posture.

## Why This Package Exists
WSHEDIMPL24 migrated `case12.for` transition continuation into detach closure,
but residual unresolved-detachment behavior remained reachable when WS20 was
enabled without WS21 activation. That lane is not baseline-authoritative process
migration and must be closed to keep WS20 opt-in routing on migrated
detach-capacity pathways.

## Scope
### Included
- Canonical contract/index updates for WSHEDIMPL25 closure scope:
  - `SC-ROUTE-001` (`GAP-ROUTE-009` narrative update),
  - `SC-SED-001` (`GAP-SED-006` narrative update),
  - `SC-SYSTEM-001` (`GAP-SYSTEM-008` narrative update),
  - `docs/specifications/science-contracts/index.md` summary updates.
- Contract-derived WS11 vectors for:
  - WS20-only opt-in fail-closed missing-`crfrac` behavior,
  - WS20-only opt-in routed success with `crfrac` projection and migrated WS21
    branch activity,
  - default-off continuity for existing diagnostics publication family.
- WS10 runtime updates in
  `crates/openwepp-watershed-orchestrator/src/lib.rs` to auto-activate WS21
  routing under WS20 opt-in lanes.
- Validation gate execution and evidence updates.

### Explicitly Out of Scope
- Dispositioning `GAP-ROUTE-009`, `GAP-SED-006`, or `GAP-SYSTEM-008` to
  `closed`.
- Full `chnero/chnrt/detach` parity closure beyond WS20/WS21 opt-in branch
  activation behavior covered here.

## Deliverables
1. `artifacts/wshedimpl25-contract-implementation-evidence.md`
2. `artifacts/wshedimpl25-contract-test-implementation-evidence.md`
3. `artifacts/wshedimpl25-preimplementation-contract-gate.md`
4. `artifacts/wshedimpl25-implementation-and-test-evidence.md`
5. `artifacts/wshedimpl25-kernel-profile-compliance-checklist.md`
6. `artifacts/wshedimpl25-channel-branch-payload-seam-report.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/wshedimpl25_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Amend canonical contract/index language for WSHEDIMPL25 scope.
2. Implement contract-derived WS11 vectors for WS25 behavior.
3. Record pre-implementation contract-gate evidence.
4. Implement production runtime/kernel edits.

## Autonomous Execution Intent (Required)
This package executes end-to-end through disposition without requesting
additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts explicitly label `Static:` and/or `Ran:`.

## Provenance and Authority Posture
- Canonical authority remains in
  `docs/specifications/science-contracts/contracts/SC-*.md`.
- Legacy provenance anchor defaults to
  `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Process-physics authority for this package is sourced from:
  - `src/chnrt.for`
  - `src/case12.for`
  - `src/case34.for`
  - `src/detach.for`
  - `src/enddet.for`
  - `src/dcap.for`
  - `src/convrt.for`
  - `src/cpart1.inc`
  - `src/cchprt.inc`

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl24-case12-deposition-detach-transition-migration-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl24-case12-deposition-detach-transition-migration-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/wepp-forest_260430_baseline/src/chnrt.for`
- `/workdir/wepp-forest_260430_baseline/src/case12.for`
- `/workdir/wepp-forest_260430_baseline/src/case34.for`
- `/workdir/wepp-forest_260430_baseline/src/detach.for`
- `/workdir/wepp-forest_260430_baseline/src/enddet.for`
- `/workdir/wepp-forest_260430_baseline/src/dcap.for`
- `/workdir/wepp-forest_260430_baseline/src/convrt.for`
- `/workdir/wepp-forest_260430_baseline/src/cpart1.inc`
- `/workdir/wepp-forest_260430_baseline/src/cchprt.inc`

## Intended Write Set
- `docs/work-packages/20260527-wshedimpl25-ws20-opt-in-ws21-auto-activation-closure-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-watershed-orchestrator/src/lib.rs`
- `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm WSHEDIMPL25 authorization from WSHEDIMPL24 worker handoff.

### Phase B - Contract/test preparation
- Amend canonical gap/index language for WSHEDIMPL25 scope.
- Add WS25 contract-derived WS11 vectors for WS20-only opt-in
  fail-closed/success behavior.
- Record pre-implementation contract-gate evidence.

### Phase C - Runtime and kernel implementation
- Auto-activate WS21 routing branch behavior in WS20 opt-in lanes.
- Preserve fail-closed `crfrac` seam posture for migrated WS20/WS21 closure
  path.
- Preserve typed guard and no-surrogate posture.

### Phase D - Validation and evidence
- Run required validation gates.
- Record outcomes with explicit `Static`/`Ran` labeling.

### Phase E - Disposition and handoff
- Publish updated HOLD disposition with explicit residual blocker ownership.
- Hand off next immediate actions for remaining channel sediment parity closure
  families.

## Exit Criteria
- WS20-only opt-in lanes no longer use unresolved-detachment fallback behavior.
- WS20-only opt-in lane without `crfrac` fails closed with typed guard.
- WS20-only opt-in lane with `crfrac` executes migrated WS21 routing closure and
  passes contract vectors.
- Required validation gates are executed and recorded.
- Residual blockers remain explicit with accurate GO/HOLD posture.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local kernel/test/docs updates only; no network or credential
  surface changes.
