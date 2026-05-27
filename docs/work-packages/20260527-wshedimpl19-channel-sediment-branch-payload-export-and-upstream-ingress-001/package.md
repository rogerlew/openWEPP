# 20260527-wshedimpl19-channel-sediment-branch-payload-export-and-upstream-ingress-001

## Status
- state: package-complete-with-hold
- date: 2026-05-27
- timezone: UTC
- decision: HOLD

## Objective
Execute WSHEDIMPL19 immediate-next-action scope from WSHEDIMPL18 by adding
fail-closed WS10 channel sediment branch payload export and upstream
channel-dependency ingress continuity for class-aware sediment aggregation.

## Why This Package Exists
WSHEDIMPL18 closed transport-capacity authority migration (`shield`/`trncap`)
but left full channel segment-process parity open. The next required seam was
to publish channel class payload families and consume upstream channel payloads
before full `case12/case34/detach/dcap/enddet` routine migration.

## Scope
### Included
- WS10 channel branch payload export:
  - `ws10_channel_{id}_particle_class_count`
  - `ws10_channel_{id}_particle_flow_fraction_{class:04}`
  - `ws10_channel_{id}_particle_diameter_m_{class:04}`
- WS10 upstream channel-dependency sediment ingress into class-aware load
  aggregation used by `qsed`/`tc` publication.
- WS11 contract-derived vectors for:
  - payload-family export closure,
  - upstream channel-dependency ingress continuity.
- Canonical contract/index updates for `SC-ROUTE-001`, `SC-SED-001`,
  `SC-SYSTEM-001`, and `science-contracts/index.md`.

### Explicitly Out of Scope
- Full segment-loop parity migration families:
  `case12/case34/detach/dcap/enddet`.
- Full `chnero/chnrt` inflow-partition parity closure/disposition.
- Closure of `GAP-ROUTE-009`, `GAP-SED-006`, `GAP-SYSTEM-008`.

## Deliverables
1. `artifacts/wshedimpl19-contract-implementation-evidence.md`
2. `artifacts/wshedimpl19-contract-test-implementation-evidence.md`
3. `artifacts/wshedimpl19-preimplementation-contract-gate.md`
4. `artifacts/wshedimpl19-implementation-and-test-evidence.md`
5. `artifacts/wshedimpl19-kernel-profile-compliance-checklist.md`
6. `artifacts/wshedimpl19-channel-branch-payload-seam-report.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/wshedimpl19_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Amend canonical contract language for WSHEDIMPL19 scope.
2. Implement contract-derived tests for payload export/ingress vectors.
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
- Full channel sediment parity remains open and non-promotable until unresolved
  segment-loop families are migrated and validated.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl18-channel-sediment-transport-capacity-authority-migration-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl18-channel-sediment-transport-capacity-authority-migration-001/artifacts/worker-handoff.md`
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

## Intended Write Set
- `docs/work-packages/20260527-wshedimpl19-channel-sediment-branch-payload-export-and-upstream-ingress-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-watershed-orchestrator/src/lib.rs`
- `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm WSHEDIMPL19 authorization from WSHEDIMPL18 worker handoff.

### Phase B - Contract/test preparation
- Amend canonical gap/index text for WS19 seam scope.
- Add WS11 contract-derived vectors for payload export/ingress.
- Record pre-implementation contract gate.

### Phase C - Runtime and kernel implementation
- Export WS10 channel class payload families with typed guard continuity.
- Ingest upstream channel dependency payloads in class-aware aggregation.

### Phase D - Validation and evidence
- Run required validation gates.
- Record outcomes with explicit `Static`/`Ran` labeling.

### Phase E - Disposition and handoff
- Publish disposition with explicit residual HOLD ownership.
- Hand off full segment-loop migration wave.

## Exit Criteria
- WS10 publishes channel class payload families for downstream use.
- WS10 ingests upstream channel payloads for class-aware aggregation.
- WS11 vectors enforce export/ingress seam continuity.
- Canonical contracts/index capture WS19 scope and residual blocker posture.
- Required validation gates are executed and recorded.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local kernel/test/docs updates only; no network or credential
  surface changes.

## Execution Outcome Summary
- Implemented WS19 runtime seam:
  - added fail-closed channel class payload export,
  - added upstream channel-dependency payload ingress into class-aware
    aggregation for `qsed`/`tc` publication continuity.
- Added WS11 vectors:
  - `wshedimpl19_contract_channel_exports_class_payload_family`
  - `wshedimpl19_contract_channel_ingresses_upstream_channel_payload`
- Updated canonical contract/index posture:
  - `SC-ROUTE-001` revision `21`
  - `SC-SED-001` revision `20`
  - `SC-SYSTEM-001` revision `42`
- Residual process-physics blockers remain open by design
  (`GAP-ROUTE-009`, `GAP-SED-006`, `GAP-SYSTEM-008`).
