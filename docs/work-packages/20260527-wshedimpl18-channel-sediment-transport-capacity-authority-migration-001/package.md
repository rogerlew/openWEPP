# 20260527-wshedimpl18-channel-sediment-transport-capacity-authority-migration-001

## Status
- state: package-complete-with-hold
- date: 2026-05-27
- timezone: UTC
- decision: HOLD

## Objective
Execute WSHEDIMPL18 immediate-next-action scope from WSHEDIMPL17 by replacing
the WS11 channel sediment surrogate `tc = qsed` path with baseline-authoritative
transport-capacity lineage (`shield`/`trncap` + channel hydraulics coupling)
using existing WS15/WS16/WS17 runtime seam families.

## Why This Package Exists
WSHEDIMPL17 closed the final scaffold seam needed before substantive channel
sediment migration. The remaining blocker rows (`GAP-SYSTEM-008`,
`GAP-ROUTE-009`, `GAP-SED-006`) still reflected surrogate sediment publication
behavior. Immediate next action required beginning full `chnero/chnrt/detach`
migration with process-equation vectors, starting with transport-capacity
authority.

## Scope
### Included
- WS10 channel sediment publication migration from surrogate identity
  (`tc = qsed`) to class-aware transport-capacity computation using migrated
  baseline formulas (`shield`/`trncap`) and channel hydraulic coupling.
- Runtime aggregation of contributor class payload families
  (`particle_flow_fraction`, `particle_diameter_m`, mass payloads) for channel
  transport-capacity evaluation.
- Contract/index updates for `SC-ROUTE-001`, `SC-SED-001`, `SC-SYSTEM-001`,
  and registry `index.md` capturing WSHEDIMPL18 closure and residual blockers.
- Contract-derived WS11 vectors proving:
  - `tc` publication no longer collapses to `qsed`,
  - transport-capacity output responds to class-diameter payload changes.

### Explicitly Out of Scope
- Full segment-loop parity migration for remaining channel sediment families:
  `case12/case34/detach/dcap/enddet` and complete `chnero/chnrt` top/lateral
  inflow partition semantics.
- Closure/disposition of `GAP-SYSTEM-008`, `GAP-ROUTE-009`, `GAP-SED-006` to
  `closed`.
- Watershed comparator lane promotion for final parity disposition.

## Deliverables
1. `artifacts/wshedimpl18-contract-implementation-evidence.md`
2. `artifacts/wshedimpl18-contract-test-implementation-evidence.md`
3. `artifacts/wshedimpl18-preimplementation-contract-gate.md`
4. `artifacts/wshedimpl18-implementation-and-test-evidence.md`
5. `artifacts/wshedimpl18-kernel-profile-compliance-checklist.md`
6. `artifacts/wshedimpl18-channel-transport-capacity-migration-report.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/wshedimpl18_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Amend canonical contract language for WSHEDIMPL18 scope and residual blocker
   posture.
2. Implement contract-derived tests for transport-capacity migration vectors.
3. Record pre-implementation contract gate evidence.
4. Implement production runtime/kernel edits.

## Autonomous Execution Intent (Required)
This package is execution-ready and must execute end-to-end through disposition
without requesting additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts explicitly label `Static:` and/or `Ran:`.

## Provenance and Authority Posture
- Canonical authority remains in
  `docs/specifications/science-contracts/contracts/SC-*.md`.
- Legacy provenance anchor defaults to
  `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Migrated equations/constants in scope are baseline-authoritative
  `shield`/`trncap`/`hydchn` lineage.
- Full channel sediment process parity remains open and non-promotable until
  remaining segment-loop families are migrated and validated.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl17-channel-segment-geometry-hydraulic-seam-intake-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl17-channel-segment-geometry-hydraulic-seam-intake-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/wepp-forest_260430_baseline/src/chnero.for`
- `/workdir/wepp-forest_260430_baseline/src/chnrt.for`
- `/workdir/wepp-forest_260430_baseline/src/trncap.for`
- `/workdir/wepp-forest_260430_baseline/src/hydchn.for`
- `/workdir/wepp-forest_260430_baseline/src/shield.for`

## Intended Write Set
- `docs/work-packages/20260527-wshedimpl18-channel-sediment-transport-capacity-authority-migration-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-watershed-orchestrator/src/lib.rs`
- `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm WSHEDIMPL18 authorization from WSHEDIMPL17 worker handoff immediate
  next actions.

### Phase B - Contract/test preparation
- Update canonical gap/index language for WSHEDIMPL18 transport-capacity
  closure posture.
- Add contract-derived WS11 vectors for process-equivalence behavior.
- Record pre-implementation contract gate.

### Phase C - Runtime and kernel implementation
- Replace WS11 surrogate `tc = qsed` with class-aware transport-capacity
  computation using migrated baseline `shield`/`trncap` lineage and channel
  hydraulic coupling.
- Preserve WS10 fail-closed guard posture for required channel/class payload
  symbols and domain constraints.

### Phase D - Validation and evidence
- Run formatter and required validation gates.
- Record outcomes with explicit `Static`/`Ran` labeling.

### Phase E - Disposition and handoff
- Publish disposition with explicit residual HOLD ownership.
- Hand off remaining segment-loop migration wave for `case12/case34/detach/
  dcap/enddet` + full `chnero/chnrt` parity closure.

## Exit Criteria
- WS10 channel `tc` publication is computed through migrated transport-capacity
  lineage rather than surrogate identity coupling.
- WS11 contract vectors verify:
  - `tc` differs from `qsed` under nominal seeded lane,
  - `tc` responds to class-diameter payload perturbation while `qsed` remains
    mass-conserved.
- Canonical contract/index posture records WSHEDIMPL18 closure and residual
  non-promotable blockers.
- Required validation gates are executed and recorded.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local kernel/test/docs updates only; no network or credential
  surface changes.

## Execution Outcome Summary
- Implemented WSHEDIMPL18 transport-capacity migration wave:
  - WS10 channel sediment path now uses class-aware payload aggregation and
    migrated baseline `shield`/`trncap` transport-capacity equations with
    channel-hydraulic coupling to publish `tc`.
  - Surrogate identity `tc = qsed` is removed from WS10 production path.
- Added WS11 contract-derived vectors confirming process behavior (`tc != qsed`
  under nominal lane and `tc` sensitivity to particle-diameter payload changes).
- Canonical contract/index posture updated to record WSHEDIMPL18 closure while
  preserving residual non-promotable blocker ownership for remaining
  segment-loop routine migration families.
