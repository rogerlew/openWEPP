# 20260527-wshedimpl27-enddet-bracket-closure-001

## Status
- state: complete
- date: 2026-05-27
- timezone: UTC
- decision: HOLD

## Objective
Execute WSHEDIMPL27 immediate-next-action scope from WSHEDIMPL26 by migrating
baseline-authoritative `enddet.for` bracket-update semantics (`xdsmal/xdbig`
search progression) in WS21 case4 enddet closure lanes, while preserving typed
guard posture and explicit residual parity blocker ownership.

## Why This Package Exists
After WSHEDIMPL26, full `chnero/chnrt/detach` parity still includes enddet
search-iteration branch behavior from baseline `enddet.for`. Existing runtime
performed a reduced loop that did not preserve the full `xdbig` re-bracketing
path, leaving this branch family incompletely migrated.

## Scope
### Included
- Canonical contract/index updates for WSHEDIMPL27 scope:
  - `SC-ROUTE-001` (`GAP-ROUTE-009`),
  - `SC-SED-001` (`GAP-SED-006`),
  - `SC-SYSTEM-001` (`GAP-SYSTEM-008`),
  - `docs/specifications/science-contracts/index.md`.
- Contract-derived WS11 vectors for WS27 enddet bracket behavior.
- WS10 runtime updates in
  `crates/openwepp-watershed-orchestrator/src/lib.rs` implementing baseline
  `enddet.for` `xdbig/xdsmal` bracket progression semantics in case4 enddet
  closure.
- Validation gate execution and evidence updates.

### Explicitly Out of Scope
- Dispositioning `GAP-ROUTE-009`, `GAP-SED-006`, or `GAP-SYSTEM-008` to
  `closed`.
- Full remaining `chnero/chnrt/detach` parity closure beyond enddet bracket
  behavior covered here.

## Deliverables
1. `artifacts/wshedimpl27-contract-implementation-evidence.md`
2. `artifacts/wshedimpl27-contract-test-implementation-evidence.md`
3. `artifacts/wshedimpl27-preimplementation-contract-gate.md`
4. `artifacts/wshedimpl27-implementation-and-test-evidence.md`
5. `artifacts/wshedimpl27-kernel-profile-compliance-checklist.md`
6. `artifacts/wshedimpl27-channel-branch-payload-seam-report.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/wshedimpl27_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Amend canonical contract/index language for WSHEDIMPL27 scope.
2. Implement contract-derived WS11 vectors for WS27 behavior.
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
  - `src/enddet.for`
  - `src/detach.for`
  - `src/dcap.for`
  - `src/cpart1.inc`
  - `src/cchprt.inc`

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl26-detach-dcap-flagm2-iterative-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl26-detach-dcap-flagm2-iterative-closure-001/artifacts/worker-handoff.md`
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
- `/workdir/wepp-forest_260430_baseline/src/enddet.for`
- `/workdir/wepp-forest_260430_baseline/src/detach.for`
- `/workdir/wepp-forest_260430_baseline/src/dcap.for`
- `/workdir/wepp-forest_260430_baseline/src/cpart1.inc`
- `/workdir/wepp-forest_260430_baseline/src/cchprt.inc`

## Intended Write Set
- `docs/work-packages/20260527-wshedimpl27-enddet-bracket-closure-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-watershed-orchestrator/src/lib.rs`
- `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm WSHEDIMPL27 authorization from WSHEDIMPL26 worker handoff.

### Phase B - Contract/test preparation
- Amend canonical gap/index language for WS27 scope.
- Add WS27 contract-derived WS11 vectors for enddet bracket behavior.
- Record pre-implementation contract-gate evidence.

### Phase C - Runtime and kernel implementation
- Migrate `enddet.for` `xdbig/xdsmal` bracket progression semantics in WS21
  case4 enddet closure lanes.
- Preserve typed guard and no-surrogate posture.

### Phase D - Validation and evidence
- Run required validation gates.
- Record outcomes with explicit `Static`/`Ran` labeling.

### Phase E - Disposition and handoff
- Publish updated HOLD disposition with explicit residual blocker ownership.
- Hand off next immediate actions for remaining channel sediment parity closure
  families.

## Exit Criteria
- WS21 enddet closure path preserves baseline `xdbig` re-bracketing semantics.
- WS27 contract-derived vectors pass and preserve typed guard posture.
- Required validation gates are executed and recorded.
- Residual blockers remain explicit with accurate GO/HOLD posture.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local kernel/test/docs updates only; no network or credential
  surface changes.
