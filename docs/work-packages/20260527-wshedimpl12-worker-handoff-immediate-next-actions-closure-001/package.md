# 20260527-wshedimpl12-worker-handoff-immediate-next-actions-closure-001

## Status
- state: package-complete-with-hold
- date: 2026-05-27
- timezone: UTC
- decision: HOLD

## Objective
Execute WSHEDIMPL12 by completing the WSHEDIMPL11 worker-handoff immediate next
actions operationally: produce execution-ready follow-on package specifications
for the residual blockers (`GAP-SYSTEM-007`, `GAP-SYSTEM-005`, and
`GAP-SYSTEM-008`), run baseline validation commands to anchor starting posture,
and publish explicit sequencing/ownership.

## Why This Package Exists
WSHEDIMPL11 closed reduced-family active projection but left three residual
blockers explicitly called out in handoff. Those actions required immediate
operational closure so downstream execution can proceed without ambiguity.

## Scope
### Included
- Convert WSHEDIMPL11 handoff immediate actions into explicit follow-on package
  specifications with objective, scope, write set, phase plan, required
  reading, and validation gates.
- Publish dependency-ordered execution sequence for:
  - full active-lane 15-function parity migration (`GAP-SYSTEM-007`),
  - baseline-authoritative watershed comparator lane closure (`GAP-SYSTEM-005`),
  - full channel sediment process-parity migration (`GAP-SYSTEM-008`).
- Run required workspace validation gates to confirm clean starting posture
  before downstream package execution.
- Update package queue visibility and WSHEDIMPL11 handoff references.

### Explicitly Out of Scope
- Implementing production kernel/runtime code for active-lane 15-function
  parity.
- Implementing production watershed comparator harness code.
- Implementing production channel sediment process-parity kernel migration.

## Deliverables
1. `artifacts/wshedimpl12-follow-on-package-specs.md`
2. `artifacts/wshedimpl12-watershed-validation-and-comparator-rerun-report.md`
3. `artifacts/wshedimpl12-hold-lift-decision-report.md`
4. `artifacts/wshedimpl12-contract-implementation-evidence.md`
5. `artifacts/wshedimpl12-contract-test-implementation-evidence.md`
6. `artifacts/wshedimpl12-preimplementation-contract-gate.md`
7. `artifacts/wshedimpl12-implementation-and-test-evidence.md`
8. `artifacts/wshedimpl12-kernel-profile-compliance-checklist.md`
9. `artifacts/owned-file-manifest.md`
10. `artifacts/gate-results.md`
11. `artifacts/wshedimpl12_disposition.md`
12. `artifacts/worker-handoff.md`
13. `artifacts/review_agent_a.md`
14. `artifacts/review_agent_b.md`
15. `artifacts/verification_agent_a.md`
16. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. Confirm canonical contract authority and current residual gap posture in
   `SC-IMPOUND-001`, `SC-ROUTE-001`, `SC-SED-001`, and `SC-SYSTEM-001`.
2. Define contract-derived closure expectations for each follow-on package.
3. Record pre-implementation contract gate evidence for follow-on execution.
4. Defer production edits to the downstream execution packages.

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
- No heuristic physics substitutions are permitted in downstream production
  migration packages.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl11-runtime-active-structure-coefficient-projection-001/package.md`
- `/workdir/openWEPP/docs/work-packages/20260527-wshedimpl11-runtime-active-structure-coefficient-projection-001/artifacts/worker-handoff.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-IMPOUND-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/wepp-forest_260430_baseline/src/impint.for`
- `/workdir/wepp-forest_260430_baseline/src/imphnw.for`
- `/workdir/wepp-forest_260430_baseline/src/impflo.for`
- `/workdir/wepp-forest_260430_baseline/src/wshchr.for`
- `/workdir/wepp-forest_260430_baseline/src/chnero.for`
- `/workdir/wepp-forest_260430_baseline/src/chnrt.for`
- `/workdir/wepp-forest_260430_baseline/src/detach.for`

## Intended Write Set
- `docs/work-packages/20260527-wshedimpl12-worker-handoff-immediate-next-actions-closure-001/**`
- `docs/work-packages/README.md`
- `docs/work-packages/20260527-wshedimpl11-runtime-active-structure-coefficient-projection-001/artifacts/worker-handoff.md`

## Phase Plan
### Phase A - Intake and scope freeze
- Confirm WSHEDIMPL11 handoff immediate action rows and current blocker IDs.

### Phase B - Follow-on package specification authoring
- Author execution-ready follow-on package specs for `GAP-SYSTEM-007`,
  `GAP-SYSTEM-005`, and `GAP-SYSTEM-008`.
- Include explicit required-reading and contract-first sequence requirements in
  each spec.

### Phase C - Validation and governance evidence
- Run required workspace gates.
- Record truthful `Static`/`Ran` evidence and publish queue updates.

### Phase D - Disposition and handoff
- Publish HOLD/GO posture for WSHEDIMPL12 scope and downstream execution order.

## Exit Criteria
- WSHEDIMPL11 immediate next actions are fully translated into executable
  follow-on package specs with no ambiguity in ownership or sequencing.
- Queue/readme and handoff artifacts point to those specs.
- Required workspace gates are executed and captured.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: docs/governance updates and validation runs only.

## Execution Outcome Summary
- WSHEDIMPL11 immediate next actions were operationalized as explicit
  follow-on package specs and sequence.
- Workspace validation gates were rerun and captured for downstream kickoff
  confidence.
- Program-level watershed disposition remains `HOLD` pending execution of the
  follow-on migration packages for `GAP-SYSTEM-007`, `GAP-SYSTEM-005`, and
  `GAP-SYSTEM-008`.
