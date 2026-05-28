# 20260528-wshedimpl30-erodible-shape-transition-fallback-mapping-001

## Status
- state: complete
- date: 2026-05-28
- timezone: UTC
- decision: HOLD

## Objective
Execute WSHEDIMPL30 immediate-next-action scope from WSHEDIMPL29 by migrating
baseline-authoritative erodible-channel shape-transition continuity in
`chnrt/detach` WS20+WS21 lanes, including `ishape=3` pathway activation and
`depa/depb`-driven rectangular fallback mapping used by hydraulic/detachment
calls.

## Why This Package Exists
WSHEDIMPL29 closed rectangular `werb -> widb(i-1)` mutation/writeback seams,
but worker handoff identified the next closure seam: erodible-lane shape
transition continuity (`ishape=3`) with explicit `depa/depb` fallback behavior
traceable to baseline `chnrt.for` and `detach.for`.

## Scope
### Included
- Canonical contract/index updates for WSHEDIMPL30 scope:
  - `SC-ROUTE-001` (`GAP-ROUTE-009`)
  - `SC-SED-001` (`GAP-SED-006`)
  - `SC-SYSTEM-001` (`GAP-SYSTEM-008`)
  - `docs/specifications/science-contracts/index.md`
- Contract-derived WS11 vectors for erodible-lane (`ishape=3`) routing and
  `depa/depb`-driven rectangular fallback continuity.
- WS10 runtime updates in
  `crates/openwepp-watershed-orchestrator/src/lib.rs` for shape-flag mapping
  and fallback application in WS20/WS21 routing lanes.
- Validation gate execution and evidence updates.

### Explicitly Out of Scope
- Dispositioning `GAP-ROUTE-009`, `GAP-SED-006`, or `GAP-SYSTEM-008` to
  `closed`.
- Full remaining `chnero/chnrt/detach` parity closure beyond shape-transition
  fallback mapping.

## Deliverables
1. `artifacts/wshedimpl30-contract-implementation-evidence.md`
2. `artifacts/wshedimpl30-contract-test-implementation-evidence.md`
3. `artifacts/wshedimpl30-preimplementation-contract-gate.md`
4. `artifacts/wshedimpl30-implementation-and-test-evidence.md`
5. `artifacts/wshedimpl30-kernel-profile-compliance-checklist.md`
6. `artifacts/wshedimpl30-channel-shape-transition-seam-report.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/wshedimpl30_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Amend canonical contract/index language for WSHEDIMPL30 scope.
2. Implement contract-derived WS11 vectors for WSHEDIMPL30 behavior.
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
  - `src/detach.for`
  - `src/dcap.for`
  - `src/hydchn.for`
  - `src/case12.for`
  - `src/case34.for`

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl29-channel-rectangular-width-mutation-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl29-channel-rectangular-width-mutation-closure-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/wepp-forest_260430_baseline/src/chnrt.for`
- `/workdir/wepp-forest_260430_baseline/src/detach.for`
- `/workdir/wepp-forest_260430_baseline/src/dcap.for`
- `/workdir/wepp-forest_260430_baseline/src/hydchn.for`
- `/workdir/wepp-forest_260430_baseline/src/case12.for`
- `/workdir/wepp-forest_260430_baseline/src/case34.for`

## Intended Write Set
- `docs/work-packages/20260528-wshedimpl30-erodible-shape-transition-fallback-mapping-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-watershed-orchestrator/src/lib.rs`
- `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm WSHEDIMPL30 authorization from WSHEDIMPL29 worker handoff.

### Phase B - Contract/test preparation
- Amend canonical gap/index language for WSHEDIMPL30 scope.
- Add WSHEDIMPL30 contract-derived WS11 vector(s) for erodible-lane
  shape-transition/fallback behavior.
- Record pre-implementation contract-gate evidence.

### Phase C - Runtime and kernel implementation
- Migrate `chnrt/detach` shape-flag continuity semantics into WS20/WS21 routing
  paths:
  - erodible path activation (`ishape=3`),
  - `depb(i-1)` rectangular fallback for upper-boundary hydraulics/dcap,
  - `depa(i)` rectangular fallback for lower-boundary hydraulics/detach lanes.

### Phase D - Validation and evidence
- Run required validation gates.
- Record outcomes with explicit `Static`/`Ran` labeling.

### Phase E - Disposition and handoff
- Publish updated HOLD disposition with explicit residual blocker ownership.
- Hand off immediate next actions for remaining channel sediment parity
  families.

## Exit Criteria
- WS20/WS21 routing uses baseline-authoritative erodible-lane shape-flag
  continuity with `depa/depb`-driven rectangular fallback mapping.
- WSHEDIMPL30 contract-derived vectors pass and preserve typed guard posture.
- Required validation gates are executed and recorded.
- Residual blockers remain explicit with accurate GO/HOLD posture.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local kernel/test/docs updates only; no network or credential
  surface changes.
