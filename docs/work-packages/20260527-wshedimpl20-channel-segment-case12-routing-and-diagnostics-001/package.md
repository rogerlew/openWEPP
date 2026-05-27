# 20260527-wshedimpl20-channel-segment-case12-routing-and-diagnostics-001

## Status
- state: package-complete-with-hold
- date: 2026-05-27
- timezone: UTC
- decision: HOLD

## Objective
Execute WSHEDIMPL20 immediate-next-action scope from WSHEDIMPL19 by
implementing baseline-derived WS10 channel segment-loop inflow-partition +
`case12` deposition-family routing scaffolding behind explicit opt-in, while
publishing fail-closed diagnostics for unresolved detachment-family parity.

## Why This Package Exists
WSHEDIMPL19 closed channel payload export and upstream-ingress seams, but the
remaining blocker wave required real segment-loop process movement toward
`chnero/chnrt` parity. This package lands the first executable segment-loop
slice with explicit provenance and non-silent residual-family tracking.

## Scope
### Included
- WS10 channel segment-loop opt-in state family:
  - `ws10_channel_{id}_ws20_case12_enable`
- Baseline-derived segment-loop scaffolding in production kernel path:
  - inflow partition from hillslope vs dependency hydrograph peaks,
  - class fall-velocity derivation (`falvel` lineage),
  - segment hydraulic + transport-capacity coupling (`hydchn`/`trncap` lineage),
  - `case12`-family deposition branch routing equations.
- Explicit unresolved-family diagnostics publication:
  - `ws10_channel_{id}_ws20_case1_segment_count`
  - `ws10_channel_{id}_ws20_case2_segment_count`
  - `ws10_channel_{id}_ws20_detachment_unmigrated_segment_count`
- WS11 contract-derived vectors for:
  - default-off opt-in behavior,
  - opt-in diagnostics continuity for unresolved detachment branch selection.
- Canonical contract/index updates for `SC-ROUTE-001`, `SC-SED-001`,
  `SC-SYSTEM-001`, and `science-contracts/index.md`.

### Explicitly Out of Scope
- Full baseline-parity migration for detachment families:
  `case34/detach/dcap/enddet`.
- Closure/disposition of `GAP-ROUTE-009`, `GAP-SED-006`, `GAP-SYSTEM-008`.
- Final watershed comparator-lane GO disposition for channel sediment parity.

## Deliverables
1. `artifacts/wshedimpl20-contract-implementation-evidence.md`
2. `artifacts/wshedimpl20-contract-test-implementation-evidence.md`
3. `artifacts/wshedimpl20-preimplementation-contract-gate.md`
4. `artifacts/wshedimpl20-implementation-and-test-evidence.md`
5. `artifacts/wshedimpl20-kernel-profile-compliance-checklist.md`
6. `artifacts/wshedimpl20-channel-branch-payload-seam-report.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/wshedimpl20_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Amend canonical contract language for WSHEDIMPL20 scope.
2. Implement contract-derived tests for WS20 segment-loop vectors.
3. Record pre-implementation contract gate evidence.
4. Implement production runtime/kernel edits.

## Autonomous Execution Intent (Required)
This package is execution-ready and executes end-to-end through disposition
without requesting additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts explicitly label `Static:` and/or `Ran:`.

## Provenance and Authority Posture
- Canonical authority remains in
  `docs/specifications/science-contracts/contracts/SC-*.md`.
- Legacy provenance anchor defaults to
  `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Segment-loop branch closure remains non-promotable until detachment families
  are migrated (`case34/detach/dcap/enddet`) and validated end-to-end.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl19-channel-sediment-branch-payload-export-and-upstream-ingress-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl19-channel-sediment-branch-payload-export-and-upstream-ingress-001/artifacts/worker-handoff.md`
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
- `/workdir/wepp-forest_260430_baseline/src/case12.for`
- `/workdir/wepp-forest_260430_baseline/src/case34.for`
- `/workdir/wepp-forest_260430_baseline/src/detach.for`
- `/workdir/wepp-forest_260430_baseline/src/dcap.for`
- `/workdir/wepp-forest_260430_baseline/src/enddet.for`
- `/workdir/wepp-forest_260430_baseline/src/falvel.for`

## Intended Write Set
- `docs/work-packages/20260527-wshedimpl20-channel-segment-case12-routing-and-diagnostics-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-watershed-orchestrator/src/lib.rs`
- `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm WSHEDIMPL20 authorization from WSHEDIMPL19 worker handoff.

### Phase B - Contract/test preparation
- Amend canonical gap/index language for WS20 segment-loop slice scope.
- Add WS11 contract-derived vectors for WS20 default-off and opt-in diagnostics.
- Record pre-implementation contract gate.

### Phase C - Runtime and kernel implementation
- Add WS20 opt-in and segment-loop routing scaffolding in WS10 channel kernel.
- Publish explicit unresolved detachment-family diagnostics.

### Phase D - Validation and evidence
- Run required validation gates.
- Record outcomes with explicit `Static`/`Ran` labeling.

### Phase E - Disposition and handoff
- Publish disposition with explicit residual HOLD ownership.
- Hand off remaining detachment-family migration wave.

## Exit Criteria
- WS10 publishes WS20 diagnostics symbols with default-off stable behavior.
- WS20 opt-in lane executes segment-loop scaffolding and unresolved-family
  diagnostics without breaking existing WS11 conformance lanes.
- Canonical contracts/index capture WS20 landed scope and residual blockers.
- Required validation gates are executed and recorded.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local kernel/test/docs updates only; no network or credential
  surface changes.

## Execution Outcome Summary
- Implemented WS20 runtime slice:
  - added explicit WS20 opt-in symbol
    (`ws10_channel_{id}_ws20_case12_enable`);
  - added baseline-derived segment-loop scaffolding for inflow partition,
    fall-velocity computation, hydraulic/capacity coupling, and `case12`
    branch-family routing equations;
  - added unresolved detachment-family diagnostics publication with
    fail-closed counter continuity.
- Added WS11 vectors:
  - `wshedimpl20_contract_case12_routing_is_opt_in_and_defaults_to_zero_diagnostics`
  - `wshedimpl20_contract_case12_opt_in_tracks_detachment_unmigrated_diagnostics`
- Updated canonical contract/index posture while preserving non-promotable
  residual blockers:
  - `GAP-ROUTE-009`
  - `GAP-SED-006`
  - `GAP-SYSTEM-008`
