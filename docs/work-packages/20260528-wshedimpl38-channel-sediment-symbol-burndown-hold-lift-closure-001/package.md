# 20260528-wshedimpl38-channel-sediment-symbol-burndown-hold-lift-closure-001

## Status
- state: queued
- date: 2026-05-28
- timezone: UTC
- decision: HOLD

## Objective
Execute WSHEDIMPL38 as the final symbol/process burndown wave required for
watershed HOLD-lift readiness by closing residual channel-sediment
`chnero/chnrt/detach` parity blockers, resolving unresolved-diagnostic symbol
families, and producing disposition-grade evidence for
`GAP-ROUTE-009`/`GAP-SED-006`/`GAP-SYSTEM-008`.

## Why This Package Exists
After WSHEDIMPL36 (parser/runtime rating-curve closure) and WSHEDIMPL37
(hydrology route-chain closure), remaining HOLD blockers are concentrated in
channel sediment process parity and associated symbol/publication governance.
This package scopes final closure criteria and HOLD-lift evidence.

## Scope
### Included
- Canonical contract/index updates for WSHEDIMPL38:
  - `SC-ROUTE-001` (`GAP-ROUTE-009`)
  - `SC-SED-001` (`GAP-SED-006`)
  - `SC-SYSTEM-001` (`GAP-SYSTEM-008`)
  - `docs/specifications/science-contracts/index.md`
- Production runtime closure in
  `crates/openwepp-watershed-orchestrator/src/lib.rs` for residual channel
  sediment process and symbol families:
  - resolve remaining `chnero/chnrt/detach` parity branches,
  - remove/retire unresolved fallback diagnostics where closure is complete,
  - preserve explicit typed hard-fail behavior for required payload domains.
- Contract-derived vectors in
  `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs` and
  related seam tests for residual families.
- End-to-end HOLD-lift evidence lane execution:
  - `cargo fmt --check`
  - `cargo clippy --workspace --all-targets -- -D warnings`
  - `cargo test --workspace`
  - `cargo deny check`
  - watershed comparator/replay lane reruns required by active contract gates.

### Explicitly Out of Scope
- New physics/features outside pinned baseline migration scope.
- Risk acceptance of unresolved process-physics gaps.

## Deliverables
1. `artifacts/wshedimpl38-contract-implementation-evidence.md`
2. `artifacts/wshedimpl38-contract-test-implementation-evidence.md`
3. `artifacts/wshedimpl38-preimplementation-contract-gate.md`
4. `artifacts/wshedimpl38-implementation-and-test-evidence.md`
5. `artifacts/wshedimpl38-kernel-profile-compliance-checklist.md`
6. `artifacts/wshedimpl38-channel-sediment-symbol-burndown-seam-report.md`
7. `artifacts/wshedimpl38-hold-lift-burndown-matrix.md`
8. `artifacts/owned-file-manifest.md`
9. `artifacts/gate-results.md`
10. `artifacts/wshedimpl38_disposition.md`
11. `artifacts/worker-handoff.md`
12. `artifacts/review_agent_a.md`
13. `artifacts/review_agent_b.md`
14. `artifacts/verification_agent_a.md`
15. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Amend canonical contract/index language for WSHEDIMPL38 scope.
2. Implement contract-derived residual-family vectors.
3. Record pre-implementation contract-gate evidence.
4. Implement production residual-family closure edits.

## Autonomous Execution Intent (Required)
This package executes end-to-end through disposition without requesting
additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must explicitly label `Static:` and/or `Ran:`.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260528-wshedimpl37-ws11-route-chain-wshcqi-wshirs-wshrun-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260528-wshedimpl37-ws11-route-chain-wshcqi-wshirs-wshrun-closure-001/artifacts/worker-handoff.md`

## Intended Write Set
- `docs/work-packages/20260528-wshedimpl38-channel-sediment-symbol-burndown-hold-lift-closure-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-watershed-orchestrator/src/lib.rs`
- `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`
- comparator/replay harness files touched by required hold-lift vectors

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm WSHEDIMPL38 authorization from WSHEDIMPL37 handoff.

### Phase B - Contract/test preparation
- Amend canonical gap/index language for WSHEDIMPL38 scope.
- Add WSHEDIMPL38 contract-derived closure vectors.
- Record pre-implementation contract-gate evidence.

### Phase C - Residual closure migration
- Implement residual `chnero/chnrt/detach` parity closures and symbol
  governance cleanup.

### Phase D - HOLD-lift evidence run
- Execute full required validation/comparator gates and record exact outcomes.

### Phase E - Disposition and handoff
- Publish GO/HOLD disposition with explicit closed/open gap map and next-action
  queue if any residual blockers remain.

## Exit Criteria
- Residual channel sediment parity blockers are either closed or explicitly
  narrowed with evidence-backed ownership.
- Gap posture for `GAP-ROUTE-009`/`GAP-SED-006`/`GAP-SYSTEM-008` is updated
  accurately with validation evidence.
- HOLD-lift readiness is explicitly concluded with truthfully labeled evidence.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local kernel runtime/test/docs updates only; no network or
  credential surface changes.
