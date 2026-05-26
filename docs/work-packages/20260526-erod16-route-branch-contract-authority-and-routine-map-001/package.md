# 20260526-erod16-route-branch-contract-authority-and-routine-map-001

## Status
- state: package-complete-with-go
- date: 2026-05-26
- timezone: UTC
- decision: GO

## Objective
Execute EROD16 by amending canonical `SC-SED-001` and `SC-ROUTE-001` to
codify baseline-authoritative hillslope `route.for` segment-branch authority
(`mshear` 1..5 and deposition/detachment branch families), preserve alias
continuity expectations, and correct `rtpart.for` provenance classification.

## Why This Package Exists
ROUTEPLAN01 published an explicit queue requiring EROD16 as the contract-first
entry point for sediment-routing closure. Existing contracts lacked explicit
canonical mapping of the `CONTIN -> ROUTE` branch family and retained an audit
lineage ambiguity around `rtpart.for`.

## Scope
### Included
- Canonical amendments to:
  - `docs/specifications/science-contracts/contracts/SC-SED-001.md`
  - `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
  - `docs/specifications/science-contracts/index.md`
- Baseline-authoritative routine mapping for:
  - `contin.for`, `route.for`, `xcrit.for`, `depc.for`, `depend.for`,
    `depos.for`, `erod.for`, `enrich.for`
- Provenance correction for `rtpart.for` classification using `grow.for`
  call-site evidence.
- Governance, review, verification, and disposition artifacts for EROD17
  handoff readiness.

### Explicitly Out of Scope
- Contract-derived test implementation (EROD17 scope).
- Runtime state-topology and production code edits (EROD18+ scope).
- Parity reruns and hold-lift disposition (EROD21 scope).

## Deliverables
1. `artifacts/erod16-contract-authority-amendment-log.md`
2. `artifacts/erod16-route-routine-authority-map.md`
3. `artifacts/erod16-cross-contract-gap-disposition.md`
4. `artifacts/erod16-contract-implementation-evidence.md`
5. `artifacts/erod16-contract-test-implementation-evidence.md`
6. `artifacts/erod16-preimplementation-contract-gate.md`
7. `artifacts/erod16-implementation-and-test-evidence.md`
8. `artifacts/erod16-kernel-profile-compliance-checklist.md`
9. `artifacts/owned-file-manifest.md`
10. `artifacts/gate-results.md`
11. `artifacts/erod16_disposition.md`
12. `artifacts/worker-handoff.md`
13. `artifacts/review_agent_a.md`
14. `artifacts/review_agent_b.md`
15. `artifacts/verification_agent_a.md`
16. `artifacts/verification_agent_b.md`

## Mandatory Contract-First Sequence (Required)
For downstream code-authoring packages:
1. implement canonical contract amendments,
2. implement contract-derived tests,
3. record pre-implementation contract gate evidence,
4. modify production code.

EROD16 executes step 1 for sediment-routing branch-family migration scope.

## Autonomous Execution Intent (Required)
This package is execution-ready and complete for end-to-end autonomous
execution through disposition without additional user intervention unless
hard-blocked.

## Truthfulness Labeling Requirement
All evidence artifacts include explicit `Static:` and/or `Ran:` labels.

## Provenance and Authority Posture
- Canonical authority remains in `SC-*` contract files.
- Baseline-authoritative migration source:
  `/workdir/wepp-forest_260430_baseline@dac3c950d8b16cc73774bf5ce2e7e11f80baac70`.
- No heuristic/proxy process-physics substitutions are allowed.

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/docs/work-packages/20260526-routeplan01-hillslope-sediment-routing-assessment-and-queue-001/artifacts/sediment-routing-wp-queue.md`
- `/workdir/openWEPP/docs/specifications/science-contract-authoring-procedure.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/kernel-process-contract-profile.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/index.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-SYSTEM-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-RUNOFFPART-001.md`
- `/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-HYDRAULICS-001.md`
- `/workdir/openWEPP/docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `/workdir/openWEPP/docs/decisions/0012-legacy-wepp-260430-baseline-anchor.md`
- `/workdir/wepp-forest_260430_baseline/src/contin.for`
- `/workdir/wepp-forest_260430_baseline/src/route.for`
- `/workdir/wepp-forest_260430_baseline/src/xcrit.for`
- `/workdir/wepp-forest_260430_baseline/src/depc.for`
- `/workdir/wepp-forest_260430_baseline/src/depend.for`
- `/workdir/wepp-forest_260430_baseline/src/depos.for`
- `/workdir/wepp-forest_260430_baseline/src/erod.for`
- `/workdir/wepp-forest_260430_baseline/src/enrich.for`
- `/workdir/wepp-forest_260430_baseline/src/rtpart.for`
- `/workdir/wepp-forest_260430_baseline/src/grow.for`

## Intended Write Set
- `docs/specifications/science-contracts/contracts/SC-SED-001.md`
- `docs/specifications/science-contracts/contracts/SC-ROUTE-001.md`
- `docs/specifications/science-contracts/index.md`
- `docs/work-packages/README.md`
- `docs/work-packages/20260526-erod16-route-branch-contract-authority-and-routine-map-001/**`

## Phase Plan
### Phase A - Intake and authority freeze
- Confirm ROUTEPLAN01 queue authorization and EROD16 objective boundaries.

### Phase B - Contract authority amendment
- Amend canonical contracts to encode explicit hillslope route routine lineage,
  branch invariants, and alias continuity requirements.

### Phase C - Cross-contract scope partition and provenance correction
- Correct routine-domain provenance (`rtpart.for`) and clarify route-domain
  ownership boundaries between `SC-SED-001` and `SC-ROUTE-001`.

### Phase D - Governance closeout
- Complete required evidence, review, verification, and handoff artifacts.

## Exit Criteria
- `SC-SED-001` contains explicit baseline route branch-family authority and
  alias continuity requirements.
- `SC-ROUTE-001` contains explicit scope partitioning that classifies
  hillslope `CONTIN -> ROUTE` lineage under `SC-SED-001`.
- `rtpart.for` provenance correction is canonicalized.
- EROD17 handoff requirements are explicit in artifacts.
- Required governance artifacts are complete with truthful labels.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: contract-authoring/docs package; no production runtime mutation.

## Execution Outcome Summary
- Canonical contracts now encode baseline hillslope route-branch authority,
  including routine-chain lineage and `mshear` branch invariants.
- Provenance classification now explicitly excludes `rtpart.for` from routing
  branch authority.
- Downstream queue entry signal for EROD17 is satisfied.
