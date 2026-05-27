# 20260527-wshedimpl11-runtime-active-structure-coefficient-projection-001

## Status
- state: package-complete-with-hold
- date: 2026-05-27
- timezone: UTC
- decision: HOLD

## Objective
Execute WSHED11 by implementing runtime active-structure coefficient projection
from exported branch payloads into WS12 runtime coefficient families for
watershed impoundment seeding (`GAP-IMPOUND-006`, `GAP-SYSTEM-007`).

## Why This Package Exists
WSHED10 closed parser export coverage for active structure payload families and
left runtime projection as the remaining blocker. The current seam still fails
closed whenever active structure flags are present, blocking active-lane WS12
execution on parser-authoritative inputs.

## Scope
### Included
- Implement active-structure runtime coefficient projection in
  `seed_watershed_runtime_surface_from_watershed_impoundment` /
  `derive_ws12_impoundment_coefficients` using exported payloads.
- Add contract-derived tests that prove active-structure fixtures now seed
  runtime coefficient surfaces without manual/synthetic injection.
- Preserve typed fail-closed behavior for non-finite/domain-invalid projected
  coefficients and unsupported malformed payload surfaces.
- Amend canonical contracts/index notes to reflect WSHED11 scope and evidence.
- Run required kernel gates and publish package artifacts through disposition.

### Explicitly Out of Scope
- Full WS12 kernel outflow-family expansion from reduced scalar coefficient
  families into 15-structure function parity.
- Channel sediment parity closure (`GAP-SYSTEM-008` / `GAP-ROUTE-009` /
  `GAP-SED-006`).
- Baseline-authoritative watershed end-to-end comparator lane closure
  (`GAP-SYSTEM-005`).

## Deliverables
1. `artifacts/wshedimpl11-watershed-validation-and-comparator-rerun-report.md`
2. `artifacts/wshedimpl11-hold-lift-decision-report.md`
3. `artifacts/wshedimpl11-contract-implementation-evidence.md`
4. `artifacts/wshedimpl11-contract-test-implementation-evidence.md`
5. `artifacts/wshedimpl11-preimplementation-contract-gate.md`
6. `artifacts/wshedimpl11-implementation-and-test-evidence.md`
7. `artifacts/wshedimpl11-kernel-profile-compliance-checklist.md`
8. `artifacts/owned-file-manifest.md`
9. `artifacts/gate-results.md`
10. `artifacts/wshedimpl11_disposition.md`
11. `artifacts/worker-handoff.md`
12. `artifacts/review_agent_a.md`
13. `artifacts/review_agent_b.md`
14. `artifacts/verification_agent_a.md`
15. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Amend canonical contract gap language (`SC-IMPOUND-001`, `SC-SYSTEM-001`,
   `science-contracts/index.md`) for WSHED11 scope and intended closure
   posture.
2. Add contract-derived test coverage for active-structure runtime coefficient
   seeding and WS12 conformance vectors.
3. Record pre-implementation contract gate evidence.
4. Implement production runtime-seam projection code changes.

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
- No silent defaults/clamping for domain-invalid runtime-seam projection
  surfaces; violations must raise typed errors.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl10-active-structure-impoundment-parser-payload-export-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-IMPOUNDMENT-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
- `/workdir/openWEPP/tests/integration/ws12_impoundment_physics_equivalence_contract.rs`
- `/workdir/openWEPP/tests/fixtures/infile/watershed_impoundment/strict_valid_active_payloads.imp`
- `/workdir/wepp-forest_260430_baseline/src/impint.for`
- `/workdir/wepp-forest_260430_baseline/src/impflo.for`
- `/workdir/wepp-forest_260430_baseline/src/imphnw.for`

## Intended Write Set
- `docs/work-packages/20260527-wshedimpl11-runtime-active-structure-coefficient-projection-001/**`
- `docs/work-packages/README.md`
- `docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md`
- `docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `docs/specifications/science-contracts/index.md`
- `crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`
- `tests/integration/ws12_impoundment_physics_equivalence_contract.rs`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm WSHED11 authorization from WSHED10 handoff and queue posture.

### Phase B - Contract and test preparation
- Apply canonical contract amendments for WSHED11 closure target.
- Add contract-derived active projection test coverage.
- Record pre-implementation contract gate evidence.

### Phase C - Runtime seam implementation
- Implement active-structure coefficient projection from exported payloads into
  runtime WS12 coefficient families.
- Maintain typed guard behavior for non-finite/domain-invalid projections.

### Phase D - Validation and governance evidence
- Run required tests and gates.
- Update package artifacts with truthful `Static`/`Ran` sections.

### Phase E - Disposition and handoff
- Publish GO/HOLD decision and residual follow-on scope.

## Exit Criteria
- Active impoundment fixture seeds runtime coefficient symbols without
  `WS-RUNTIME-E-012` projection-gap failure.
- Active projection contract vectors pass in runtime-input and WS12 integration
  test surfaces.
- Required gates are executed and captured.
- Canonical gap language and index notes are updated for post-WSHED11 posture.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: runtime-seam math/docs/tests only; no external connectivity or
  credential surface changes.

## Execution Outcome Summary
- WSHED11 implemented runtime active-structure coefficient projection from
  exported parser payloads into required WS12 runtime coefficient families.
- Active runtime seeding no longer fails closed on parser-authoritative active
  fixtures; active vectors now execute in runtime-input and WS12 integration
  test lanes.
- Canonical contract gap language and index notes were updated to reflect
  removal of projection-gap fail-closed behavior and residual non-promotable
  scope for full active-lane 15-function parity.
- Program-level watershed disposition remains `HOLD` due blockers outside
  WSHED11 scope (`GAP-SYSTEM-005`, `GAP-SYSTEM-007` residual parity scope, and
  `GAP-SYSTEM-008`).
