# 20260527-wshedimpl21-channel-case34-enddet-routing-and-detach-gate-001

## Status
- state: complete
- date: 2026-05-27
- timezone: UTC
- decision: HOLD

## Objective
Execute WSHEDIMPL21 immediate-next-action scope from WSHEDIMPL20 by adding WS21
opt-in diagnostics scaffolding for `case34/enddet` branch tracking in the WS20
segment-loop runtime path, while preserving explicit fail-visible handling for
unmigrated baseline-authoritative `detach/dcap` process-family surfaces.

## Why This Package Exists
WSHEDIMPL20 landed `case12`-family routing and diagnostics but retained
detachment-family migration blockers. WSHEDIMPL21 advances closure by wiring
WS21 branch-level diagnostics and opt-in controls that isolate residual
`detach/dcap` migration ownership and provide deterministic visibility for
follow-on authoritative closure.

## Scope
### Included
- Runtime WS21 opt-in diagnostics scaffolding for WS10 segment-loop case34/enddet
  branch-family tracking.
- WS21 opt-in gate symbol for case34/enddet branch activation:
  - `ws10_channel_{id}_ws21_case34_enable`
- WS21 diagnostics publication symbols:
  - `ws10_channel_{id}_ws21_case3_segment_count`
  - `ws10_channel_{id}_ws21_case4_segment_count`
  - `ws10_channel_{id}_ws21_enddet_segment_count`
  - `ws10_channel_{id}_ws21_detach_unmigrated_segment_count`
- WS11 contract-derived vectors covering:
  - default-off WS21 behavior,
  - opt-in WS21 case34/enddet diagnostics and continuity.
- Canonical contract/index updates in:
  - `SC-ROUTE-001`
  - `SC-SED-001`
  - `SC-SYSTEM-001`
  - `science-contracts/index.md`

### Explicitly Out of Scope
- Full baseline-authoritative migration of `detach.for` and `dcap.for`
  production math paths where missing state-family authority remains.
- Closure/disposition of:
  - `GAP-ROUTE-009`
  - `GAP-SED-006`
  - `GAP-SYSTEM-008`
- Final comparator-lane GO disposition for watershed channel sediment parity.

## Deliverables
1. `artifacts/wshedimpl21-contract-implementation-evidence.md`
2. `artifacts/wshedimpl21-contract-test-implementation-evidence.md`
3. `artifacts/wshedimpl21-preimplementation-contract-gate.md`
4. `artifacts/wshedimpl21-implementation-and-test-evidence.md`
5. `artifacts/wshedimpl21-kernel-profile-compliance-checklist.md`
6. `artifacts/wshedimpl21-channel-branch-payload-seam-report.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/wshedimpl21_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Amend canonical contract/index language for WSHEDIMPL21 scope.
2. Implement contract-derived WS11 vectors for WS21 behavior.
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
- Remaining `detach/dcap` migration surfaces stay explicitly non-promotable and
  must remain fail-visible in runtime diagnostics.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl20-channel-segment-case12-routing-and-diagnostics-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl20-channel-segment-case12-routing-and-diagnostics-001/artifacts/worker-handoff.md`
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
- `/workdir/wepp-forest_260430_baseline/src/case34.for`
- `/workdir/wepp-forest_260430_baseline/src/enddet.for`
- `/workdir/wepp-forest_260430_baseline/src/detach.for`
- `/workdir/wepp-forest_260430_baseline/src/dcap.for`

## Intended Write Set
- `docs/work-packages/20260527-wshedimpl21-channel-case34-enddet-routing-and-detach-gate-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-watershed-orchestrator/src/lib.rs`
- `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm WSHEDIMPL21 authorization from WSHEDIMPL20 worker handoff.

### Phase B - Contract/test preparation
- Amend canonical gap/index language for WS21 landed scope.
- Add WS11 contract-derived vectors for WS21 default-off/opt-in behavior.
- Record pre-implementation contract gate evidence.

### Phase C - Runtime and kernel implementation
- Add WS21 opt-in gate and case34/enddet diagnostics scaffolding in WS10
  channel runtime path.
- Publish WS21 diagnostics and preserve explicit unresolved `detach/dcap`
  visibility.

### Phase D - Validation and evidence
- Run required validation gates.
- Record outcomes with explicit `Static`/`Ran` labeling.

### Phase E - Disposition and handoff
- Publish disposition with explicit residual HOLD ownership.
- Hand off remaining `detach/dcap` migration wave.

## Exit Criteria
- WS10 publishes WS21 diagnostics symbols with default-off stable behavior.
- WS21 opt-in lane publishes case34/enddet diagnostics continuity without
  regressing existing WS11/WS20 vectors.
- Residual `detach/dcap` gaps remain explicit and fail-visible.
- Canonical contracts/index capture WS21 landed scope and residual blockers.
- Required validation gates are executed and recorded.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local kernel/test/docs updates only; no network or credential
  surface changes.
