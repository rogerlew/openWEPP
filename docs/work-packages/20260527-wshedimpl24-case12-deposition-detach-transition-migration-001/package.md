# 20260527-wshedimpl24-case12-deposition-detach-transition-migration-001

## Status
- state: complete
- date: 2026-05-27
- timezone: UTC
- decision: HOLD

## Objective
Execute WSHEDIMPL24 immediate-next-action scope from WSHEDIMPL23 by migrating
baseline-authoritative `case12.for` deposition-to-detachment transition
behavior (`xdemax < x(i)` continuation into `detach.for`) for WS20 segment-loop
routing, adding explicit transition diagnostics publication, and preserving
typed guard posture.

## Why This Package Exists
WSHEDIMPL23 closed WS21 `case4 -> detach` iterative closure ownership, but
remaining `chnero/chnrt` parity families still include uncovered `case12.for`
transition behavior where deposition ends within a segment and routing must
continue through baseline-authoritative `detach.for` closure over the remaining
subsegment. WS20 currently does not expose this transition as explicit migrated
runtime behavior with dedicated diagnostics.

## Scope
### Included
- Canonical contract/index updates for WSHEDIMPL24 transition-closure scope:
  - `SC-ROUTE-001` (`GAP-ROUTE-009` narrative update),
  - `SC-SED-001` (`GAP-SED-006` narrative update),
  - `SC-SYSTEM-001` (`GAP-SYSTEM-008` narrative update),
  - `docs/specifications/science-contracts/index.md` summary updates.
- Contract-derived WS11 vectors for:
  - explicit WS20 `case12 -> detach` transition branch execution coverage,
  - explicit publication continuity for `ws24_case2_detach_segment_count`.
- WS10 runtime implementation updates in
  `crates/openwepp-watershed-orchestrator/src/lib.rs` to migrate
  baseline-authoritative `case12.for` transition into `detach.for` when
  `xdemax < x(i)` in WS20 path.
- Validation-gate execution and evidence updates, including watershed
  baseline-authoritative comparator-lane rerun evidence.

### Explicitly Out of Scope
- Dispositioning `GAP-ROUTE-009`, `GAP-SED-006`, or `GAP-SYSTEM-008` to
  `closed`.
- Full `chnero/chnrt/detach` parity closure claims beyond this WS24 transition
  branch migration slice.

## Deliverables
1. `artifacts/wshedimpl24-contract-implementation-evidence.md`
2. `artifacts/wshedimpl24-contract-test-implementation-evidence.md`
3. `artifacts/wshedimpl24-preimplementation-contract-gate.md`
4. `artifacts/wshedimpl24-implementation-and-test-evidence.md`
5. `artifacts/wshedimpl24-kernel-profile-compliance-checklist.md`
6. `artifacts/wshedimpl24-channel-branch-payload-seam-report.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/wshedimpl24_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Amend canonical contract/index language for WSHEDIMPL24 scope.
2. Implement contract-derived WS11 vectors for WS24 behavior.
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
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl23-channel-detach-case4-iterative-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl23-channel-detach-case4-iterative-closure-001/artifacts/worker-handoff.md`
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
- `docs/work-packages/20260527-wshedimpl24-case12-deposition-detach-transition-migration-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-watershed-orchestrator/src/lib.rs`
- `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm WSHEDIMPL24 authorization from WSHEDIMPL23 worker handoff.

### Phase B - Contract/test preparation
- Amend canonical gap/index language for WSHEDIMPL24 scope.
- Add WS24 contract-derived WS11 vectors for `case12 -> detach` transition
  branch coverage and diagnostics publication continuity.
- Record pre-implementation contract-gate evidence.

### Phase C - Runtime and kernel implementation
- Implement baseline-authoritative `case12.for` transition behavior for
  `xdemax < x(i)` continuation into `detach.for`.
- Publish explicit WS24 transition diagnostics symbol:
  `ws10_channel_{id}_ws24_case2_detach_segment_count`.
- Preserve typed guard and no-surrogate posture.

### Phase D - Validation and evidence
- Run required validation gates.
- Rerun watershed baseline-authoritative comparator lane.
- Record outcomes with explicit `Static`/`Ran` labeling.

### Phase E - Disposition and handoff
- Publish updated HOLD disposition with explicit residual blocker ownership.
- Hand off next immediate actions for remaining channel sediment parity
  closure families.

## Exit Criteria
- WS20 `case12 -> detach` transition branch (`xdemax < x(i)`) executes migrated
  baseline-authoritative continuation behavior through `detach.for`.
- WS24 diagnostics publication symbol
  `ws10_channel_{id}_ws24_case2_detach_segment_count` is present and covered by
  contract-derived vector(s).
- Required validation gates are executed and recorded.
- Watershed baseline-authoritative comparator-lane rerun evidence is updated.
- Residual blockers remain explicit with accurate GO/HOLD posture.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local kernel/test/docs updates only; no network or credential
  surface changes.
