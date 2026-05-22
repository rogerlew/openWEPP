# 20260522-clim11-climate-ownership-boundary-reconciliation-001

## Status
- state: queued
- date: 2026-05-22
- timezone: UTC

## Objective
Reconcile climate forcing ownership boundaries between hillslope and watershed
orchestration surfaces, with explicit ADR-level routing authority and seam
contracts.

## Why This Package Exists
CLIM04 accepted review finding `CLIM04-RVW-003`, which identified mismatch
between watershed orchestration narrative and climate forcing ownership
behavior. This package closes that architecture ambiguity before further runtime
refactors.

## Scope
### Included
- Authoritative reconciliation of watershed-vs-hillslope climate forcing
  ownership and routing.
- Updates to architecture/disposition references to remove conflicting
  narratives.
- Contract-level acceptance criteria for where climate adaptation is allowed to
  occur.

### Explicitly Out of Scope
- Shared-runtime extraction/refactor implementation work (`CLIM12`).
- Typed forcing surface refactor (`CLIM13`) beyond ownership contract
  requirements.

## Deliverables
1. Ownership reconciliation artifact:
   - `artifacts/climate-ownership-boundary-contract.md`
2. Package governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/clim11_disposition.md`
3. Dual review/verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/README.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-clim04-breakpoint-runtime-port-and-policy-reconciliation-001/artifacts/clim04_disposition.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-arch21-architecture-review-re-closeout-001/`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/`
- `/home/workdir/openWEPP/crates/openwepp-watershed-orchestrator/`

## Intended Write Set
- `docs/decisions/**` (if ADR updates are required)
- `docs/work-packages/20260522-clim11-climate-ownership-boundary-reconciliation-001/**`
- `crates/openwepp-hillslope-orchestrator/**` (if ownership transfer requires)
- `crates/openwepp-watershed-orchestrator/**` (if ownership transfer requires)

## Phase Plan
### Phase 0 - Intake
- Confirm current climate ownership assumptions in code and architecture docs.

### Phase 1 - Contract Reconciliation
- Publish authoritative ownership boundary and accepted routing model.

### Phase 2 - Verification
- Ensure references and ownership claims are internally consistent.

### Phase 3 - Disposition
- Capture evidence, reviews, and final disposition.

## Exit Criteria
- Ownership boundary is explicit and non-conflicting across architecture and
  climate work-package docs.
- Any code ownership relocations required by the boundary are implemented or
  explicitly queued.
- Artifacts include clear evidence-mode labeling (`Static:` vs `Ran:`).
