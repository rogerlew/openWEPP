# 20260527-wshedimpl04-watershed-runtime-seam-closure-001

## Status
- state: package-complete-with-hold
- date: 2026-05-27
- timezone: UTC
- decision: HOLD

## Objective
Execute WSHED04 by closing watershed parser-to-runtime seam projection for
impoundment coefficient/state families required by WS12 and removing
synthetic/manual coefficient seeding from watershed contract vectors.

## Why This Package Exists
WSHEDIMPL02 completed WSHED03 contract-derived vectors and pre-implementation
gate evidence, including the expected-failure vector
`wshed03_contract_ws12_vector_requires_parser_projected_coefficients_without_manual_seed`.
The current production seam still requires synthetic/manual WS12 coefficient
injection. WSHED04 closes that seam before WSHED05/06/07 migration packages.

## Scope
### Included
- Runtime seam closure in watershed runtime-input adaptation so parsed
  impoundment payloads project required WS12 coefficient families into runtime
  state symbols:
  - `a,b,c,d,e,ha,ht,hlm,a0,a1,a2,l0,l1,l2`
- Typed fail-closed guard enforcement for missing/non-finite/out-of-domain seam
  derivation values (no silent defaults/clamping for required projection
  surfaces).
- Contract-derived test updates removing manual coefficient seeding dependency
  in WS10/WS11/WS12 integration surfaces.
- Promotion of WSHED03 WS12 parser-projection vector from ignored expected
  failure to active conformance test.
- Package governance artifacts, dual review/verification, and disposition.

### Explicitly Out of Scope
- Full WS12 RK4/adaptive regime-transition migration (`GAP-IMPOUND-005`,
  WSHED07 scope).
- Full WS11 wave-routing physics migration (`GAP-ROUTE-008`, WSHED05 scope).
- Channel sediment migration (`GAP-ROUTE-009`, `GAP-SED-006`, WSHED06 scope).
- Watershed parquet writer activation (`GAP-SYSTEM-006`, WSHED08 scope).

## Deliverables
1. `artifacts/wshedimpl04-runtime-seam-closure-report.md`
2. `artifacts/wshedimpl04-contract-implementation-evidence.md`
3. `artifacts/wshedimpl04-contract-test-implementation-evidence.md`
4. `artifacts/wshedimpl04-preimplementation-contract-gate.md`
5. `artifacts/wshedimpl04-implementation-and-test-evidence.md`
6. `artifacts/wshedimpl04-kernel-profile-compliance-checklist.md`
7. `artifacts/owned-file-manifest.md`
8. `artifacts/gate-results.md`
9. `artifacts/wshedimpl04_disposition.md`
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

WSHEDIMPL04 executes steps 2, 3, and 4 for runtime seam closure; step 1 is
limited to minimal canonical gap-status updates only when closure evidence
warrants them.

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
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedplan01-watershed-channel-routing-orchestration-parquet-assessment-001/artifacts/watershed-channel-routing-orchestration-parquet-wp-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl02-watershed-contract-derived-tests-and-preimplementation-gate-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl02-watershed-contract-derived-tests-and-preimplementation-gate-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
- `/workdir/openWEPP/crates/openwepp-runner/src/bin/openwepp-cli-watershed.rs`
- `/workdir/openWEPP/tests/integration/ws10_watershed_kernel_contract.rs`
- `/workdir/openWEPP/tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`
- `/workdir/openWEPP/tests/integration/ws12_impoundment_physics_equivalence_contract.rs`
- `/workdir/wepp-forest_260430_baseline/src/imphnw.for`
- `/workdir/wepp-forest_260430_baseline/src/impflo.for`
- `/workdir/wepp-forest_260430_baseline/src/impmai.for`

## Intended Write Set
- `docs/work-packages/20260527-wshedimpl04-watershed-runtime-seam-closure-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
- `tests/integration/ws10_watershed_kernel_contract.rs`
- `tests/integration/ws11_channel_routing_physics_equivalence_contract.rs`
- `tests/integration/ws12_impoundment_physics_equivalence_contract.rs`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm WSHED04 queue authority and prior WSHED03 expected-failure baseline.

### Phase B - Contract-derived test and gate preparation
- Remove manual WS12 coefficient seeding from contract test scaffolds.
- Promote WS12 parser-projection vector from ignored expected-failure to active
  conformance status.

### Phase C - Runtime seam implementation
- Implement parser-to-runtime impoundment coefficient projection in watershed
  runtime-input adaptation.
- Enforce typed fail-closed guards for non-finite/out-of-domain seam values and
  unsupported projection surfaces.

### Phase D - Validation and governance evidence
- Run scoped integration tests and required package gates.
- Update kernel-profile checklist, evidence artifacts, dual review, and dual
  verification artifacts.

### Phase E - Disposition and handoff
- Record closure status for WSHED04 seam scope and explicit follow-on routing
  to WSHED05/06/07/08/09.

## Exit Criteria
- WS12 required coefficient symbols are projected by production runtime seam for
  parser-authoritative impoundment payloads.
- No manual coefficient seeding remains in WS10/WS11/WS12 integration tests.
- WS12 parser-projection vector executes as active conformance (not ignored).
- Missing/non-finite/domain-invalid seam projection inputs fail closed with
  typed errors.
- Required artifacts are complete with truthful evidence labels.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: runtime seam projection + tests; no network/credential scope.

## Execution Outcome Summary
- WSHED04 runtime seam closure is implemented for inactive-structure
  impoundment lanes:
  - production runtime seeding now projects required WS12 coefficient families
    (`a,b,c,d,e,ha,ht,hlm,a0,a1,a2,l0,l1,l2`) from parser-authoritative
    impoundment curve payloads,
  - manual/synthetic coefficient seeding dependency was removed from WS10/WS11/
    WS12 integration surfaces,
  - WSHED03 WS12 parser-projection vector is now active and passing.
- Seam posture is fail-closed for unsupported active outlet-structure projection
  domains; runtime emits typed seam errors instead of silent defaults/clamping.
- Canonical contract gap statements (`GAP-IMPOUND-006`, `GAP-SYSTEM-007`) were
  updated to reflect closure of manual-seeding dependency and narrowed residual
  blocker scope.
- Program-level watershed closure remains `HOLD` pending WSHED05/06/07/08/09.
