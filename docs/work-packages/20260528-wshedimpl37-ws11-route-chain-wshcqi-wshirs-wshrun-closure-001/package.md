# 20260528-wshedimpl37-ws11-route-chain-wshcqi-wshirs-wshrun-closure-001

## Status
- state: complete
- date: 2026-05-28
- timezone: UTC
- decision: HOLD

## Objective
Execute WSHEDIMPL37 as the second HOLD-lift burndown wave by migrating
baseline-authoritative WS11 hydrology routine-chain behavior
(`wshcqi/wshirs/wshrun`) into openWEPP production runtime paths and eliminating
remaining route-chain symbol/parity blockers under `GAP-ROUTE-008`.

## Why This Package Exists
WSHEDIMPL36 closes parser/runtime rating-curve control seams, but HOLD-lift
still requires hydrology routine-chain parity closure. Canonical contracts keep
`GAP-ROUTE-008` non-promotable until route-chain execution matches pinned
baseline authority with explicit typed guard and branch continuity.

## Scope
### Included
- Canonical contract/index updates for WSHEDIMPL37 traceability:
  - `SC-ROUTE-001` (`GAP-ROUTE-008`, companion `GAP-ROUTE-009` continuity)
  - `SC-SED-001` (`GAP-SED-006` trace linkage only)
  - `SC-SYSTEM-001` (`GAP-SYSTEM-008` trace linkage only)
  - `docs/specifications/science-contracts/index.md`
- Production runtime migration in
  `crates/openwepp-watershed-orchestrator/src/lib.rs` for baseline-authoritative
  route-chain behavior:
  - runon assembly (`rvolat`/`rvotop`/`rvolon`) and duration max rule,
  - channel runoff/transmission-loss branch semantics before outlet routing,
  - explicit `ipeak` branch execution continuity for WS11 route lanes.
- Contract-derived route-chain vectors in
  `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`.
- Validation gates and evidence update.

### Explicitly Out of Scope
- Full channel sediment process parity closure for `chnero/chnrt/detach`,
  owned by WSHEDIMPL38.
- Final HOLD-lift disposition closure unless all downstream blockers are
  demonstrably closed.

## Deliverables
1. `artifacts/wshedimpl37-contract-implementation-evidence.md`
2. `artifacts/wshedimpl37-contract-test-implementation-evidence.md`
3. `artifacts/wshedimpl37-preimplementation-contract-gate.md`
4. `artifacts/wshedimpl37-implementation-and-test-evidence.md`
5. `artifacts/wshedimpl37-kernel-profile-compliance-checklist.md`
6. `artifacts/wshedimpl37-ws11-route-chain-migration-seam-report.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/wshedimpl37_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Amend canonical contract/index language for WSHEDIMPL37 scope.
2. Implement contract-derived WS11 route-chain vectors.
3. Record pre-implementation contract-gate evidence.
4. Implement production route-chain migration edits.

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
- `/workdir/openWEPP/docs/work-packages/20260528-wshedimpl36-parser-runtime-rating-curve-lineage-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260528-wshedimpl36-parser-runtime-rating-curve-lineage-closure-001/artifacts/worker-handoff.md`

## Intended Write Set
- `docs/work-packages/20260528-wshedimpl37-ws11-route-chain-wshcqi-wshirs-wshrun-closure-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-watershed-orchestrator/src/lib.rs`
- `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm WSHEDIMPL37 authorization from WSHEDIMPL36 handoff.

### Phase B - Contract/test preparation
- Amend canonical gap/index language for WSHEDIMPL37 scope.
- Add WSHEDIMPL37 contract-derived WS11 route-chain vectors.
- Record pre-implementation contract-gate evidence.

### Phase C - Route-chain migration
- Implement baseline-authoritative `wshcqi/wshirs/wshrun` route-chain behavior
  in production watershed runtime lanes.

### Phase D - Validation and evidence
- Run required validation gates and record outcomes with `Static`/`Ran` labels.

### Phase E - Disposition and handoff
- Publish HOLD posture update and hand off residual channel-sediment closure to
  WSHEDIMPL38.

## Exit Criteria
- WS11 route-chain execution is baseline-authoritative for covered branches.
- `GAP-ROUTE-008` is either dispositioned or narrowed with explicit residuals.
- WSHEDIMPL37 vectors pass and required validation gates are executed.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local kernel runtime/test/docs updates only; no network or
  credential surface changes.
