# 20260528-wshedimpl31-detach-lower-boundary-width-mutation-closure-001

## Status
- state: complete
- date: 2026-05-28
- timezone: UTC
- decision: HOLD

## Objective
Execute WSHEDIMPL31 immediate-next-action scope from WSHEDIMPL30 by migrating
baseline-authoritative lower-boundary width mutation semantics from
`detach.for` into WS23/WS24 closure paths:
`if flagc=2 and wera(i)>wfl then wida(i)=wera(i)`.

## Why This Package Exists
WSHEDIMPL30 closed erodible shape-transition fallback mapping (`ishape=3` with
`depa/depb` rectangular fallback). The next blocker in worker handoff is
lower-boundary detach width mutation continuity (`wera -> wida(i)`).

## Scope
### Included
- Canonical contract/index updates for WSHEDIMPL31 scope:
  - `SC-ROUTE-001` (`GAP-ROUTE-009`)
  - `SC-SED-001` (`GAP-SED-006`)
  - `SC-SYSTEM-001` (`GAP-SYSTEM-008`)
  - `docs/specifications/science-contracts/index.md`
- Contract-derived WS11 vectors for rectangular lower-boundary width mutation
  continuity and non-rectangular control behavior.
- WS10 runtime updates in
  `crates/openwepp-watershed-orchestrator/src/lib.rs` to:
  - propagate WS23/WS24 detach outcomes including `wera`-equivalent eroded
    width,
  - apply lower-boundary width mutation rule under rectangular lane semantics,
  - project/update `wida_{point:04}` state writeback symbols.
- Validation gate execution and evidence updates.

### Explicitly Out of Scope
- Dispositioning `GAP-ROUTE-009`, `GAP-SED-006`, or `GAP-SYSTEM-008` to
  `closed`.
- Parser/runtime shape-code lineage reconciliation from WSHEDIMPL30 worker
  handoff item 2 (follow-on ownership).
- Full remaining `chnero/chnrt/detach` parity closure.

## Deliverables
1. `artifacts/wshedimpl31-contract-implementation-evidence.md`
2. `artifacts/wshedimpl31-contract-test-implementation-evidence.md`
3. `artifacts/wshedimpl31-preimplementation-contract-gate.md`
4. `artifacts/wshedimpl31-implementation-and-test-evidence.md`
5. `artifacts/wshedimpl31-kernel-profile-compliance-checklist.md`
6. `artifacts/wshedimpl31-detach-lower-boundary-width-mutation-seam-report.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/wshedimpl31_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Amend canonical contract/index language for WSHEDIMPL31 scope.
2. Implement contract-derived WS11 vectors for WSHEDIMPL31 behavior.
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
- `/workdir/openWEPP/docs/work-packages/20260528-wshedimpl30-erodible-shape-transition-fallback-mapping-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260528-wshedimpl30-erodible-shape-transition-fallback-mapping-001/artifacts/worker-handoff.md`
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
- `docs/work-packages/20260528-wshedimpl31-detach-lower-boundary-width-mutation-closure-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-watershed-orchestrator/src/lib.rs`
- `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm WSHEDIMPL31 authorization from WSHEDIMPL30 worker handoff.

### Phase B - Contract/test preparation
- Amend canonical gap/index language for WSHEDIMPL31 scope.
- Add WSHEDIMPL31 contract-derived WS11 vectors.
- Record pre-implementation contract-gate evidence.

### Phase C - Runtime and kernel implementation
- Migrate lower-boundary detach width mutation semantics into WS20/WS21 paths:
  - carry WS23/WS24 `wera`-equivalent outcome from detach closure routines,
  - apply baseline rule
    (`flagc=2 && wera(i)>wfl => wida(i)=wera(i)`) at lower boundary,
  - publish `wida_{point:04}` updates through WS10 writeback.

### Phase D - Validation and evidence
- Run required validation gates.
- Record outcomes with explicit `Static`/`Ran` labeling.

### Phase E - Disposition and handoff
- Publish updated HOLD disposition with explicit residual blocker ownership.
- Hand off immediate next actions for remaining channel sediment parity
  families.

## Exit Criteria
- WS23/WS24 closure paths enforce baseline-authoritative lower-boundary width
  mutation semantics for rectangular lanes.
- WS10 state publication includes updated `wida_{point:04}` projection in WS20
  opt-in routing execution.
- WSHEDIMPL31 contract-derived vectors pass and preserve typed guard posture.
- Required validation gates are executed and recorded.
- Residual blockers remain explicit with accurate GO/HOLD posture.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local kernel/test/docs updates only; no network or credential
  surface changes.
