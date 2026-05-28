# 20260527-wshedimpl26-detach-dcap-flagm2-iterative-closure-001

## Status
- state: complete
- date: 2026-05-27
- timezone: UTC
- decision: HOLD

## Objective
Execute WSHEDIMPL26 immediate-next-action scope from WSHEDIMPL25 by
implementing baseline-authoritative `dcap.for` `flagm=2` semantics used by
`detach.for` iterative closure (`nt2 < cnpart` lane), while recording explicit
remaining branch-obligation mapping for unresolved `chnero/chnrt/detach`
families.

## Why This Package Exists
WSHEDIMPL25 closed residual WS20-only opt-in fallback behavior, but remaining
full `chnero/chnrt/detach` parity still includes iterative detach closure
dependence on `dcap(flagm=2)` max-detachment limiter behavior. Current runtime
used a flagm1-only `dcap` helper across iterative closure paths, leaving this
baseline-authoritative branch family incompletely migrated.

## Scope
### Included
- Canonical contract/index updates for WS26 scope with explicit
  branch-obligation mapping under:
  - `SC-ROUTE-001` (`GAP-ROUTE-009`),
  - `SC-SED-001` (`GAP-SED-006`),
  - `SC-SYSTEM-001` (`GAP-SYSTEM-008`),
  - `docs/specifications/science-contracts/index.md`.
- Contract-derived WS11 vectors for WS26 iterative closure behavior.
- WS10 runtime updates in
  `crates/openwepp-watershed-orchestrator/src/lib.rs` to add explicit
  `dcap(flagm=2)` behavior in WS23 iterative closure lanes.
- Validation gate execution and evidence updates.

### Explicitly Out of Scope
- Dispositioning `GAP-ROUTE-009`, `GAP-SED-006`, or `GAP-SYSTEM-008` to
  `closed`.
- Full remaining `chnero/chnrt/detach` parity closure beyond
  `dcap(flagm=2)` iterative-lane semantics covered here.

## Deliverables
1. `artifacts/wshedimpl26-contract-implementation-evidence.md`
2. `artifacts/wshedimpl26-contract-test-implementation-evidence.md`
3. `artifacts/wshedimpl26-preimplementation-contract-gate.md`
4. `artifacts/wshedimpl26-implementation-and-test-evidence.md`
5. `artifacts/wshedimpl26-kernel-profile-compliance-checklist.md`
6. `artifacts/wshedimpl26-channel-branch-payload-seam-report.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/wshedimpl26_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Amend canonical contract/index language for WSHEDIMPL26 scope.
2. Implement contract-derived WS11 vectors for WS26 behavior.
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
  - `src/dcap.for`
  - `src/enddet.for`
  - `src/cpart1.inc`
  - `src/cchprt.inc`

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl25-ws20-opt-in-ws21-auto-activation-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl25-ws20-opt-in-ws21-auto-activation-closure-001/artifacts/worker-handoff.md`
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
- `/workdir/wepp-forest_260430_baseline/src/dcap.for`
- `/workdir/wepp-forest_260430_baseline/src/enddet.for`
- `/workdir/wepp-forest_260430_baseline/src/cpart1.inc`
- `/workdir/wepp-forest_260430_baseline/src/cchprt.inc`

## Intended Write Set
- `docs/work-packages/20260527-wshedimpl26-detach-dcap-flagm2-iterative-closure-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-watershed-orchestrator/src/lib.rs`
- `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm WSHEDIMPL26 authorization from WSHEDIMPL25 worker handoff.

### Phase B - Contract/test preparation
- Amend canonical gap/index language for WS26 scope, including explicit
  residual branch-obligation mapping.
- Add WS26 contract-derived WS11 vectors for iterative closure behavior.
- Record pre-implementation contract-gate evidence.

### Phase C - Runtime and kernel implementation
- Implement `dcap(flagm=2)` max-detachment limiter semantics for WS23
  iterative closure lanes.
- Preserve typed guard and no-surrogate posture.

### Phase D - Validation and evidence
- Run required validation gates.
- Record outcomes with explicit `Static`/`Ran` labeling.

### Phase E - Disposition and handoff
- Publish updated HOLD disposition with explicit residual blocker ownership.
- Hand off next immediate actions for remaining channel sediment parity closure
  families.

## Exit Criteria
- WS23 iterative closure path no longer relies on flagm1-only `dcap`
  approximation.
- WS26 contract-derived WS11 vectors pass and preserve typed guard posture.
- Required validation gates are executed and recorded.
- Residual blockers remain explicit with accurate GO/HOLD posture.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local kernel/test/docs updates only; no network or credential
  surface changes.
