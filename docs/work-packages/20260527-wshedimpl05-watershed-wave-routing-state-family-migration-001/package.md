# 20260527-wshedimpl05-watershed-wave-routing-state-family-migration-001

## Status
- state: package-complete-with-hold
- date: 2026-05-27
- timezone: UTC
- decision: HOLD

## Objective
Execute WSHED05 by migrating WS11 `ipeak > 2` wave-routing state-family
publication (`q1`, `qin`, `qlat`, `c0..c4`) into production watershed channel
execution and promoting the matching WSHED03 expected-failure vector.

## Why This Package Exists
WSHEDIMPL03 introduced an expected-failure WS11 vector requiring canonical
wave-routing lineage state publication for `ipeak` branches 3 and 4. WSHED04
closed parser-to-runtime impoundment coefficient seams and left WSHED05 as the
next required migration step before channel sediment, impoundment continuity,
and parquet activation packages.

## Scope
### Included
- Channel runtime publication of WS11 wave-routing lineage symbols for
  `ipeak >= 3` on WS10 channel nodes:
  - `ws10_channel_{id}_q1`
  - `ws10_channel_{id}_qin`
  - `ws10_channel_{id}_qlat`
  - `ws10_channel_{id}_c0`
  - `ws10_channel_{id}_c1`
  - `ws10_channel_{id}_c2`
  - `ws10_channel_{id}_c3`
  - `ws10_channel_{id}_c4`
- Typed fail-closed guard continuity for non-finite or domain-invalid
  intermediate wave-routing state/coefficient computations.
- Promotion of
  `wshed03_contract_kw_mc_vector_requires_wave_routing_state_family_publication`
  from ignored expected-failure to active conformance.
- Package evidence, gate, review, verification, disposition, and handoff
  updates.

### Explicitly Out of Scope
- Full WS11 baseline-authoritative runon/runoff routine-chain migration
  (`wshcqi/wshirs/wshrun`) beyond this state-family publication closure slice.
- WS10 channel sediment runtime migration (`GAP-ROUTE-009`, WSHED06 scope).
- WS12 RK4/adaptive impoundment regime-transition migration (WSHED07 scope).
- Watershed parquet writer activation (WSHED08 scope).

## Deliverables
1. `artifacts/wshedimpl05-wave-routing-state-migration-report.md`
2. `artifacts/wshedimpl05-contract-implementation-evidence.md`
3. `artifacts/wshedimpl05-contract-test-implementation-evidence.md`
4. `artifacts/wshedimpl05-preimplementation-contract-gate.md`
5. `artifacts/wshedimpl05-implementation-and-test-evidence.md`
6. `artifacts/wshedimpl05-kernel-profile-compliance-checklist.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/wshedimpl05_disposition.md`
10. `artifacts/worker-handoff.md`
11. `artifacts/review_agent_a.md`
12. `artifacts/review_agent_b.md`
13. `artifacts/verification_agent_a.md`
14. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
Kernel-affecting sequencing remains mandatory:
1. implement required canonical contract amendments,
2. implement contract-derived tests,
3. record pre-implementation contract gate evidence, then
4. modify production code.

WSHEDIMPL05 executes steps 2, 3, and 4 for wave-routing state-family
publication; step 1 is limited to minimal gap-row/disposition updates if
closure evidence warrants it.

## Autonomous Execution Intent (Required)
This package is execution-ready and must proceed end-to-end through
disposition without requesting additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts must include explicit `Static:` and/or `Ran:` sections.
Do not claim command execution unless it was actually run.

## Provenance and Authority Posture
- Canonical authority is in
  `docs/specifications/science-contracts/contracts/SC-*.md`.
- Work-package artifacts are evidence and do not replace canonical authority.
- Legacy migration provenance defaults to
  `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- No heuristic/proxy process-physics substitutions are allowed in production
  migration closure claims.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedplan01-watershed-channel-routing-orchestration-parquet-assessment-001/artifacts/watershed-channel-routing-orchestration-parquet-wp-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl02-watershed-contract-derived-tests-and-preimplementation-gate-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl02-watershed-contract-derived-tests-and-preimplementation-gate-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl04-watershed-runtime-seam-closure-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl04-watershed-runtime-seam-closure-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/crates/openwepp-watershed-orchestrator/src/lib.rs`
- `/workdir/openWEPP/tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`
- `/workdir/wepp-forest_260430_baseline/src/wshcqi.for`
- `/workdir/wepp-forest_260430_baseline/src/wshdrv.for`
- `/workdir/wepp-forest_260430_baseline/src/wshrun.for`
- `/workdir/wepp-forest_260430_baseline/src/wshpek.for`
- `/workdir/wepp-forest_260430_baseline/src/wshchr.for`
- `/workdir/wepp-forest_260430_baseline/src/chrqin.for`

## Intended Write Set
- `docs/work-packages/20260527-wshedimpl05-watershed-wave-routing-state-family-migration-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-watershed-orchestrator/src/lib.rs`
- `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm queue authority and WSHED03 expected-failure baseline.

### Phase B - Contract-derived test and gate preparation
- Promote WSHED03 WS11 wave-routing state-family vector from ignored expected
  failure to active conformance.
- Record pre-implementation gate posture.

### Phase C - Runtime implementation
- Implement `ipeak > 2` wave-routing lineage state/coefficient publication in
  watershed channel production execution.
- Preserve typed fail-closed guards for non-finite/domain-invalid intermediates.

### Phase D - Validation and governance evidence
- Run scoped integration tests and required gates.
- Update kernel-profile checklist, evidence artifacts, dual review, and dual
  verification artifacts.

### Phase E - Disposition and handoff
- Record WSHED05 closure posture and explicit follow-on routing to WSHED06/07/08/09.

## Exit Criteria
- `ipeak` branches 3 and 4 publish required state-family symbols
  (`q1/qin/qlat/c0..c4`) on WS10 channel nodes.
- WSHED03 WS11 wave-routing state-family vector is active (not ignored) and
  passing.
- Wave-routing intermediate computation failures remain typed and fail-closed.
- Required artifacts are complete with truthful evidence labeling.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: local kernel behavior + tests only; no network/credential scope.

## Execution Outcome Summary
- WSHED05 scoped objective is complete:
  - WS11 `ipeak > 2` branches now publish wave-routing lineage state families
    on WS10 channel nodes (`q1/qin/qlat/c0..c4`),
  - the WSHED03 wave-routing state-family vector is promoted from ignored to
    active and passes.
- Canonical contract posture is synchronized:
  - `SC-ROUTE-001` now records WSHED05 closure of wave-state publication and
    narrows `GAP-ROUTE-008` to remaining `wshcqi/wshirs/wshrun`
    routine-chain migration.
- Program-level watershed closure remains `HOLD` pending WSHED06/07/08/09 and
  full end-to-end validation closure in WSHED09.
