# 20260527-wshedimpl23-channel-detach-case4-iterative-closure-001

## Status
- state: complete
- date: 2026-05-27
- timezone: UTC
- decision: HOLD

## Objective
Execute WSHEDIMPL23 immediate-next-action scope from WSHEDIMPL22 by migrating
baseline-authoritative `detach.for` iterative closure behavior for WS21
`case4` rows (`nt < cnpart`) in the WS20+WS21 opt-in execution lane, removing
residual WS21 unresolved-detachment fallback for that branch while preserving
typed fail-closed guard posture.

## Why This Package Exists
WSHEDIMPL22 moved WS21 into active `dcap + case34/enddet` execution but left a
residual unmigrated branch when WS21 `case4` transport-capacity checks yielded
`nt < cnpart`. That branch currently increments unresolved diagnostics and uses
fallback transport behavior. WSHEDIMPL23 closes that residual branch by porting
baseline-authoritative iterative detachment closure from `detach.for`.

## Scope
### Included
- Canonical contract/index updates for WSHEDIMPL23 branch-closure scope:
  - `SC-ROUTE-001` (`GAP-ROUTE-009` narrative update),
  - `SC-SED-001` (`GAP-SED-006` narrative update),
  - `SC-SYSTEM-001` (`GAP-SYSTEM-008` narrative update),
  - `docs/specifications/science-contracts/index.md` summary updates.
- Contract-derived WS11 vectors for:
  - WS21 opt-in branch that previously tracked unresolved detachment now
    executing without `ws21_detach_unmigrated_segment_count` increments,
  - continuity of required WS21 case-family diagnostics publication surfaces.
- WS10 runtime implementation updates in
  `crates/openwepp-watershed-orchestrator/src/lib.rs` to migrate
  baseline-authoritative `detach.for` iterative closure behavior in WS21 case4
  path (`nt < cnpart`) with typed guard/no-surrogate posture.
- Validation-gate execution and evidence updates, including watershed
  baseline-authoritative comparator-lane rerun evidence.

### Explicitly Out of Scope
- Dispositioning `GAP-ROUTE-009`, `GAP-SED-006`, or `GAP-SYSTEM-008` to
  `closed`.
- Full `chnero/chnrt/detach` parity closure claims beyond the migrated WS21
  `case4 -> detach` residual branch.

## Deliverables
1. `artifacts/wshedimpl23-contract-implementation-evidence.md`
2. `artifacts/wshedimpl23-contract-test-implementation-evidence.md`
3. `artifacts/wshedimpl23-preimplementation-contract-gate.md`
4. `artifacts/wshedimpl23-implementation-and-test-evidence.md`
5. `artifacts/wshedimpl23-kernel-profile-compliance-checklist.md`
6. `artifacts/wshedimpl23-channel-branch-payload-seam-report.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/wshedimpl23_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Amend canonical contract/index language for WSHEDIMPL23 scope.
2. Implement contract-derived WS11 vectors for WS23 behavior.
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
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl22-channel-detach-dcap-case34-enddet-migration-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl22-channel-detach-dcap-case34-enddet-migration-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/wepp-forest_260430_baseline/src/chnrt.for`
- `/workdir/wepp-forest_260430_baseline/src/case34.for`
- `/workdir/wepp-forest_260430_baseline/src/detach.for`
- `/workdir/wepp-forest_260430_baseline/src/enddet.for`
- `/workdir/wepp-forest_260430_baseline/src/dcap.for`
- `/workdir/wepp-forest_260430_baseline/src/convrt.for`
- `/workdir/wepp-forest_260430_baseline/src/cpart1.inc`
- `/workdir/wepp-forest_260430_baseline/src/cchprt.inc`

## Intended Write Set
- `docs/work-packages/20260527-wshedimpl23-channel-detach-case4-iterative-closure-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-watershed-orchestrator/src/lib.rs`
- `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm WSHEDIMPL23 authorization from WSHEDIMPL22 worker handoff.

### Phase B - Contract/test preparation
- Amend canonical gap/index language for WSHEDIMPL23 scope.
- Add WS23 contract-derived WS11 vectors for `case4 -> detach` iterative branch
  closure behavior.
- Record pre-implementation contract-gate evidence.

### Phase C - Runtime and kernel implementation
- Implement baseline-authoritative `detach.for` iterative closure behavior for
  WS21 case4 (`nt < cnpart`) in WS20+WS21 opt-in path.
- Remove residual WS21 unresolved-detachment fallback increments for that
  migrated branch.
- Preserve typed guard and no-surrogate posture.

### Phase D - Validation and evidence
- Run required validation gates.
- Rerun watershed baseline-authoritative comparator lane.
- Record outcomes with explicit `Static`/`Ran` labeling.

### Phase E - Disposition and handoff
- Publish updated HOLD disposition with explicit residual blocker ownership.
- Hand off next immediate actions for remaining channel sediment parity closure.

## Exit Criteria
- WS21 `case4 -> detach` residual branch (`nt < cnpart`) executes migrated
  baseline-authoritative iterative closure behavior.
- WS21 opt-in vectors no longer require unresolved-detachment diagnostic
  increments for that migrated branch.
- Required validation gates are executed and recorded.
- Watershed baseline-authoritative comparator-lane rerun evidence is updated.
- Residual blockers remain explicit with accurate GO/HOLD posture.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local kernel/test/docs updates only; no network or credential
  surface changes.
