# 20260526-erod20-sediment-routing-constant-symbolization-and-magic-number-elimination-001

## Status
- state: package-complete-with-hold
- date: 2026-05-26
- timezone: UTC
- decision: HOLD

## Objective
Execute EROD20 by eliminating remaining sediment-routing magic literals in the
EROD14/EROD19 production paths through named constants with explicit
provenance-style intent and wiring those constants through runtime code.

## Why This Package Exists
ROUTEPLAN01 queue item 5 requires literal cleanup after EROD19 migration so the
route branch family is no longer maintained through opaque inline numbers.

## Scope
### Included
- Constant symbolization in:
  - `crates/openwepp-hillslope-orchestrator/src/constants.rs`
  - `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support.rs`
- Governance closeout in EROD20 package artifacts and work-package index.

### Explicitly Out of Scope
- Additional route algorithm migration (`EROD19` scope).
- Final parity rerun and hold-lift disposition (`EROD21` scope).

## Deliverables
1. `artifacts/erod20-route-topology-ingress-matrix.md`
2. `artifacts/erod20-route-topology-implementation-report.md`
3. `artifacts/erod20-contract-implementation-evidence.md`
4. `artifacts/erod20-contract-test-implementation-evidence.md`
5. `artifacts/erod20-preimplementation-contract-gate.md`
6. `artifacts/erod20-implementation-and-test-evidence.md`
7. `artifacts/erod20-kernel-profile-compliance-checklist.md`
8. `artifacts/owned-file-manifest.md`
9. `artifacts/gate-results.md`
10. `artifacts/erod20_disposition.md`
11. `artifacts/worker-handoff.md`
12. `artifacts/review_agent_a.md`
13. `artifacts/review_agent_b.md`
14. `artifacts/verification_agent_a.md`
15. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
1. EROD16 contracts complete.
2. EROD17 contract-derived vectors complete.
3. EROD17 pre-implementation gate complete.
4. EROD19 runtime migration complete.
5. EROD20 literal symbolization execute.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260526-routeplan01-hillslope-sediment-routing-assessment-and-queue-001/artifacts/sediment-routing-wp-queue.md`
- `/workdir/openWEPP/docs/work-packages/20260526-erod19-route-mshear-segment-kernel-migration-001/artifacts/erod19_disposition.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SED-001.md`

## Intended Write Set
- `crates/openwepp-hillslope-orchestrator/src/constants.rs`
- `crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support.rs`
- `docs/work-packages/README.md`
- `docs/work-packages/20260526-erod20-sediment-routing-constant-symbolization-and-magic-number-elimination-001/**`

## Exit Criteria
- Targeted sediment-routing literals are replaced by named constants.
- EROD14 and EROD19 production paths compile and tests pass with constantized logic.
- EROD20 artifact set captures changed symbol families and validation evidence.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: internal constant symbolization and refactoring only.

## Execution Outcome Summary
- Introduced named constants for EROD14 case bounds, update payload sizing,
  attenuation floor, enrichment offset, and EROD19 helper thresholds/steps.
- Rewired EROD14/EROD19 code to consume named constants in place of raw literals.
- Preserved runtime behavior with passing clippy and targeted route/MOFE03 tests.
- HOLD remains until EROD21 parity rerun/disposition is complete.
