# 20260526-routeplan01-hillslope-sediment-routing-assessment-and-queue-001

## Status
- state: package-complete
- date: 2026-05-26
- timezone: UTC
- decision: GO

## Objective
Prepare and execute a baseline-authoritative assessment of the audit-flagged
partial sediment-routing support (`route.for` MSHEAR branch family), document
implementation gaps versus openWEPP, and publish a dependency-ordered
work-package queue to fully implement hillslope sediment routing and eliminate
routing magic numbers.

## Why This Package Exists
The audit row in `docs/audits/20260525_water_erosion_kernel_audit.md` marks
`route.for` support as partial and explicitly calls out that the upper-end
segment detach-vs-deposit branch family is not visible in the Rust runtime.
This package closes ambiguity by producing a direct algorithmic-gap assessment
and an execution-ready queue for full closure.

## Scope
### Included
- Static algorithmic comparison between baseline
  `/workdir/wepp-forest_260430_baseline/src/route.for` and openWEPP
  `run_erod14_wave2`/EROD13 runtime surfaces.
- Classification check for the audit row's `rtpart.for` reference.
- Explicit implementation-gap map for `route.for` segment-loop/MSHEAR behavior.
- Explicit magic-number inventory for sediment-routing runtime surfaces and a
  queue plan to replace raw literals with canonical constants.
- Dependency-ordered follow-on work-package queue artifact.
- Full governance/disposition artifacts for autonomous handoff.

### Explicitly Out of Scope
- Production Rust kernel edits.
- Canonical `SC-*` contract amendments.
- Comparator reruns and hold-lift disposition for routing parity.

## Deliverables
1. `artifacts/routeplan01-openwepp-vs-baseline-route-implementation-review.md`
2. `artifacts/sediment-routing-wp-queue.md`
3. `artifacts/routeplan01-contract-implementation-evidence.md`
4. `artifacts/routeplan01-contract-test-implementation-evidence.md`
5. `artifacts/routeplan01-preimplementation-contract-gate.md`
6. `artifacts/routeplan01-implementation-and-test-evidence.md`
7. `artifacts/routeplan01-kernel-profile-compliance-checklist.md`
8. `artifacts/owned-file-manifest.md`
9. `artifacts/gate-results.md`
10. `artifacts/routeplan01_disposition.md`
11. `artifacts/worker-handoff.md`
12. `artifacts/review_agent_a.md`
13. `artifacts/review_agent_b.md`
14. `artifacts/verification_agent_a.md`
15. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
For queued code-authoring packages:
1. implement canonical contract amendments,
2. implement contract-derived tests,
3. record pre-implementation contract gate evidence,
4. modify production code.

## Autonomous Execution Intent (Required)
This package is execution-ready and complete for assessment+queue scope without
user intervention.

## Truthfulness Labeling Requirement
All evidence artifacts use explicit `Static:` and/or `Ran:` sections.

## Provenance and Authority Posture
- Baseline-authoritative provenance source:
  `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- Canonical authority remains in `SC-*` contracts; package artifacts are
  evidence only.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/audits/20260525_water_erosion_kernel_audit.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/hydrology/03_kernel_support.rs`
- `/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/constants.rs`
- `/workdir/wepp-forest_260430_baseline/src/contin.for`
- `/workdir/wepp-forest_260430_baseline/src/route.for`
- `/workdir/wepp-forest_260430_baseline/src/depc.for`
- `/workdir/wepp-forest_260430_baseline/src/depend.for`
- `/workdir/wepp-forest_260430_baseline/src/depos.for`
- `/workdir/wepp-forest_260430_baseline/src/erod.for`
- `/workdir/wepp-forest_260430_baseline/src/enrich.for`
- `/workdir/wepp-forest_260430_baseline/src/xcrit.for`
- `/workdir/wepp-forest_260430_baseline/src/rtpart.for`

## Intended Write Set
- `docs/work-packages/20260526-routeplan01-hillslope-sediment-routing-assessment-and-queue-001/**`
- `docs/work-packages/README.md`

## Phase Plan
### Phase A - Intake and provenance freeze
- Confirm audit row target and freeze baseline authority inputs.

### Phase B - Gap assessment
- Compare `route.for` branch/control-flow shape with current Rust erosion lane.
- Capture missing routines, missing state topology, and missing branch family.
- Capture audit-reference correctness for `rtpart.for`.

### Phase C - Queue authoring
- Publish dependency-ordered queue that closes algorithmic gaps and removes
  routing magic numbers.

### Phase D - Governance closeout
- Complete review/verification/disposition artifacts for handoff.

## Exit Criteria
- Assessment artifact directly answers whether openWEPP currently implements
  `route.for` MSHEAR branch semantics.
- Queue artifact provides full closure sequence to implement segment routing and
  remove magic numbers.
- Required governance artifacts are complete with truthful evidence labels.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: planning/evidence package only; no runtime behavior change.

## Execution Outcome Summary
- `route.for` segment-loop/MSHEAR branch family is not implemented in current
  openWEPP runtime; existing Wave-2 is enrichment/class-load closure, not full
  segment routing.
- Audit row reference to `rtpart.for` as sediment-routing routine is inaccurate;
  `rtpart.for` is root-mass partitioning called from growth routines.
- A dependency-ordered queue has been authored to complete sediment routing and
  remove sediment-routing magic numbers.
