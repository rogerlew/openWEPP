# 20260527-wshedimpl22-channel-detach-dcap-case34-enddet-migration-001

## Status
- state: complete
- date: 2026-05-27
- timezone: UTC
- decision: HOLD

## Objective
Execute WSHEDIMPL22 immediate-next-action scope from WSHEDIMPL21 by replacing
WS21 opt-in unresolved-detachment fallback scaffolding with baseline-lineage
`dcap`-driven `case34/enddet` routing behavior, while preserving typed
fail-closed handling for residual unmigrated `case4 -> detach` iterative
closure paths.

## Why This Package Exists
WSHEDIMPL21 landed WS21 diagnostics and explicit unresolved visibility but left
baseline-authoritative detachment-capacity routines out of the WS21 opt-in
execution lane. WSHEDIMPL22 moves WS21 from pure diagnostics scaffolding to an
active baseline-lineage `dcap`/`case34/enddet` execution slice and introduces
required `crfrac` projection gating so the lane cannot silently run on
surrogate class-fraction assumptions.

## Scope
### Included
- WS22 contract/index updates for landed `dcap` lineage, `crfrac` gating, and
  residual blocker language.
- WS22 contract-derived WS11 vectors for:
  - required `crfrac` projection guard failure under WS20+WS21 opt-in,
  - WS20+WS21 opt-in success with projected `crfrac`.
- WS10 runtime implementation updates in watershed orchestrator:
  - required `ws10_channel_{id}_crfrac_{class:04}` intake under WS21 detachment
    branch execution,
  - baseline-lineage `dcap` helper execution in WS21 opt-in path,
  - WS21 `case34` branch routing updates and `enddet` iterative closure lane,
  - preservation of explicit unresolved diagnostics for residual `case4 ->
    detach` iterative closure not migrated in this package.
- Full validation gate execution and evidence publication.

### Explicitly Out of Scope
- Full baseline-authoritative migration of `detach.for` iterative closure
  branch for WS21 `case4` rows where `nt < cnpart`.
- Dispositioning `GAP-ROUTE-009`, `GAP-SED-006`, and `GAP-SYSTEM-008` to
  `closed`.

## Deliverables
1. `artifacts/wshedimpl22-contract-implementation-evidence.md`
2. `artifacts/wshedimpl22-contract-test-implementation-evidence.md`
3. `artifacts/wshedimpl22-preimplementation-contract-gate.md`
4. `artifacts/wshedimpl22-implementation-and-test-evidence.md`
5. `artifacts/wshedimpl22-kernel-profile-compliance-checklist.md`
6. `artifacts/wshedimpl22-channel-branch-payload-seam-report.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/wshedimpl22_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Amend canonical contract/index language for WSHEDIMPL22 scope.
2. Implement contract-derived WS11 vectors for WS22 behavior.
3. Record pre-implementation contract gate evidence.
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
  - `src/dcap.for`
  - `src/chnrt.for`
  - `src/case34.for`
  - `src/enddet.for`
  - `src/convrt.for`
  - `src/cpart1.inc`
  - `src/cchprt.inc`

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl21-channel-case34-enddet-routing-and-detach-gate-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl21-channel-case34-enddet-routing-and-detach-gate-001/artifacts/worker-handoff.md`
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
- `/workdir/wepp-forest_260430_baseline/src/dcap.for`
- `/workdir/wepp-forest_260430_baseline/src/convrt.for`
- `/workdir/wepp-forest_260430_baseline/src/cpart1.inc`
- `/workdir/wepp-forest_260430_baseline/src/cchprt.inc`

## Intended Write Set
- `docs/work-packages/20260527-wshedimpl22-channel-detach-dcap-case34-enddet-migration-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-watershed-orchestrator/src/lib.rs`
- `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm WSHEDIMPL22 authorization from WSHEDIMPL21 worker handoff.

### Phase B - Contract/test preparation
- Amend canonical gap/index language for WSHEDIMPL22 scope.
- Add WS22 contract-derived vectors for `crfrac` gate failure and success lane.
- Record pre-implementation contract gate evidence.

### Phase C - Runtime and kernel implementation
- Add WS22 `crfrac` projection-gated intake in WS21 detachment branch.
- Implement baseline-lineage `dcap` helper math and WS21 `case34/enddet` path.
- Preserve explicit unresolved diagnostics for residual `case4 -> detach`
  iterative branch not migrated in this package.

### Phase D - Validation and evidence
- Run required validation gates.
- Record outcomes with explicit `Static`/`Ran` labeling.

### Phase E - Disposition and handoff
- Publish HOLD disposition with residual blocker ownership.
- Hand off follow-on closure for remaining `case4 -> detach` iterative path.

## Exit Criteria
- WS21 opt-in path no longer uses unconditional unresolved fallback when
  `dcap`/`case34/enddet` authority inputs are present.
- WS21 opt-in path hard-fails with typed missing-input error when required
  `crfrac` class-fraction symbols are absent.
- WS11 vectors validate both failure and success WS22 lanes.
- Required validation gates are executed and recorded.
- Residual unmigrated branch ownership is explicit in disposition/handoff.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local kernel/test/docs updates only; no network or credential
  surface changes.
