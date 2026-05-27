# 20260527-wshedimpl13-active-lane-15-function-parity-migration-001

## Status
- state: package-complete-with-hold
- date: 2026-05-27
- timezone: UTC
- decision: HOLD

## Objective
Execute WSHEDIMPL13 by migrating WS12 active-lane outflow from reduced-family
projection to full legacy-authoritative 15-function min-controller composition
(`qo1..qo15`) with runtime function-family projection from exported `.imp`
branch payloads (`GAP-IMPOUND-006`, `GAP-SYSTEM-007`).

## Why This Package Exists
WSHEDIMPL11 removed projection-gap fail-closed behavior but left active-lane
parity blocked by reduced families (`a,b,c,d,e,ha,ht,hlm`). WSHEDIMPL13 closes
that residual scope by publishing full function-family symbols and routing
through legacy min-controller composition semantics.

## Scope
### Included
- Project per-function WS12 families (`f01..f15` over `a,b,c,d,e,ha`) from
  parser-exported active structure payloads in
  `seed_watershed_runtime_surface_from_watershed_impoundment`.
- Replace WS12 outflow composition in production kernel execution with
  `min(qo1,qo2,qo3) + min(qo4,qo5,qo6) + min(qo7,qo8,qo9) + qo10 + qo11 + qo12 + min(qo13,qo14,qo15)`.
- Add contract-derived tests proving active-lane min-controller composition is
  executed from projected function-family symbols.
- Amend canonical contract/index gap posture for WSHEDIMPL13 closure evidence.
- Run required workspace validation gates and publish package artifacts through
  disposition.

### Explicitly Out of Scope
- Baseline-authoritative watershed end-to-end comparator lane closure
  (`GAP-SYSTEM-005`).
- Channel sediment process-parity migration closure
  (`GAP-SYSTEM-008` / `GAP-ROUTE-009` / `GAP-SED-006`).

## Deliverables
1. `artifacts/wshedimpl13-watershed-validation-and-comparator-rerun-report.md`
2. `artifacts/wshedimpl13-hold-lift-decision-report.md`
3. `artifacts/wshedimpl13-contract-implementation-evidence.md`
4. `artifacts/wshedimpl13-contract-test-implementation-evidence.md`
5. `artifacts/wshedimpl13-preimplementation-contract-gate.md`
6. `artifacts/wshedimpl13-implementation-and-test-evidence.md`
7. `artifacts/wshedimpl13-kernel-profile-compliance-checklist.md`
8. `artifacts/owned-file-manifest.md`
9. `artifacts/gate-results.md`
10. `artifacts/wshedimpl13_disposition.md`
11. `artifacts/worker-handoff.md`
12. `artifacts/review_agent_a.md`
13. `artifacts/review_agent_b.md`
14. `artifacts/verification_agent_a.md`
15. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Amend canonical contract language (`SC-IMPOUND-001`, `SC-SYSTEM-001`,
   `science-contracts/index.md`) for WSHEDIMPL13 closure intent.
2. Add contract-derived vector coverage for 15-function active-lane outflow
   composition.
3. Record pre-implementation contract gate evidence.
4. Implement production runtime-seam + kernel changes.

## Autonomous Execution Intent (Required)
This package is execution-ready and must run end-to-end through disposition
without requesting additional user direction unless hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts include explicit `Static:` and/or `Ran:` labeling.

## Provenance and Authority Posture
- Canonical authority remains in
  `docs/specifications/science-contracts/contracts/SC-*.md`.
- Work-package artifacts are evidence and do not replace canonical authority.
- Legacy provenance anchor defaults to
  `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- No silent defaults/clamping for domain-invalid runtime/kernels surfaces;
  violations must raise typed errors.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl12-worker-handoff-immediate-next-actions-closure-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl12-worker-handoff-immediate-next-actions-closure-001/artifacts/wshedimpl12-follow-on-package-specs.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-IMPOUNDMENT-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
- `/workdir/openWEPP/crates/openwepp-watershed-orchestrator/src/lib.rs`
- `/workdir/openWEPP/tests/integration/ws12_impoundment_physics_equivalence_contract.rs`
- `/workdir/openWEPP/tests/fixtures/infile/watershed_impoundment/strict_valid_active_payloads.imp`
- `/workdir/wepp-forest_260430_baseline/src/impint.for`
- `/workdir/wepp-forest_260430_baseline/src/impflo.for`
- `/workdir/wepp-forest_260430_baseline/src/imphnw.for`

## Intended Write Set
- `docs/work-packages/20260527-wshedimpl13-active-lane-15-function-parity-migration-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
- `crates/openwepp-watershed-orchestrator/src/lib.rs`
- `tests/integration/ws12_impoundment_physics_equivalence_contract.rs`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm WSHEDIMPL13 authorization from WSHEDIMPL12 handoff.

### Phase B - Contract and test preparation
- Apply canonical contract amendments for 15-function closure posture.
- Add contract-derived min-controller vector coverage.
- Record pre-implementation contract gate evidence.

### Phase C - Runtime + kernel migration
- Implement runtime function-family projection from exported payloads.
- Implement kernel 15-function min-controller outflow composition.
- Preserve typed guard behavior for non-finite/domain-invalid states.

### Phase D - Validation and governance evidence
- Run required tests and gates.
- Update artifacts with truthful `Static`/`Ran` sections.

### Phase E - Disposition and handoff
- Publish GO/HOLD decision and residual follow-on ownership.

## Exit Criteria
- Active fixtures project `f01..f15` WS12 function families without
  `WS-RUNTIME-E-012`.
- WS12 active-lane vector proves kernel `qo` equals 15-function min-controller
  composition from projected families.
- Required gates are executed and captured.
- Canonical gap language and index notes reflect post-WSHEDIMPL13 posture.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: runtime/kernel math/docs/tests only; no credential/network changes.

## Execution Outcome Summary
- WSHEDIMPL13 implemented runtime projection of WS12 function-family payloads
  (`f01..f15`) and migrated kernel outflow composition to legacy
  min-controller semantics.
- Active-lane vector coverage confirms `qo` matches
  `min(qo1..qo3)+min(qo4..qo6)+min(qo7..qo9)+qo10+qo11+qo12+min(qo13..qo15)`.
- Canonical contracts/index were updated to close `GAP-IMPOUND-006` and
  `GAP-SYSTEM-007`.
- Program-level watershed disposition remains `HOLD` due blockers outside
  WSHEDIMPL13 scope (`GAP-SYSTEM-005`, `GAP-SYSTEM-008`).
