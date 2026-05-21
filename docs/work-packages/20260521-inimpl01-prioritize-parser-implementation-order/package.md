# 20260521-inimpl01-prioritize-parser-implementation-order

## Status
- `state`: closed
- `date`: 2026-05-21
- `timezone`: UTC
- `closed_utc`: 2026-05-21 UTC
- `outcome`: `GO_WITH_AMENDMENTS`

## Objective
Prioritize parser implementation order across all now-`active` `SC-INFILE-*`
surfaces and produce a dependency-aware execution sequence for follow-on
implementation work packages.

## Why This Package Exists
The parser contract-authoring campaign is complete and the input-surface
registry is now `active` for all governed `SC-INFILE-*` surfaces. openWEPP now
needs an explicit implementation order that optimizes for correctness-first
delivery, dependency closure, and earliest useful runnable scope.

## Scope
### Included
- Build a formal prioritization rubric for parser implementation sequencing.
- Score every `active` parser surface in
  `docs/specifications/wepp-input-files/input-surface-registry.md`.
- Define dependency-aware implementation waves (including prerequisites,
  acceptance checks, and blocking conditions).
- Identify minimum viable parser path for high-confidence early validation
  (single OFE + daily water-balance confidence tier).
- Produce follow-on implementation work-package queue proposals.
- Run dual-agent review, disposition, and dual-agent verification for this
  planning output.

### Explicitly Out of Scope
- Rust parser/runtime code implementation.
- Re-authoring `SC-INFILE-*` contracts unless a blocking contradiction is found.
- Broad comparator execution campaigns.

## Deliverables
1. Prioritization rubric:
   - `artifacts/parser-implementation-prioritization-rubric.md`
2. Surface scoring matrix:
   - `artifacts/parser-implementation-priority-matrix.csv`
3. Dependency and wave plan:
   - `artifacts/parser-implementation-wave-plan.md`
4. Canonical implementation-order plan:
   - `docs/planning/parser-implementation-order.md`
5. Follow-on work-package queue proposal:
   - `artifacts/follow-on-parser-implementation-wp-queue.md`
6. Disposition record:
   - `artifacts/inimpl01_disposition.md`
7. Review and verification evidence:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `docs/specifications/wepp-input-files/input-surface-registry.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-*.md`
- `docs/specifications/wepp-input-files/parser-contract-requirements.md`
- `docs/specifications/wepp-input-file-parser-contract-authoring-procedure.md`
- `docs/decisions/0011-architecture-first-top-down-science-contracts.md`
- `docs/work-packages/20260520-arch01-subsystem-map-and-contract-spine/artifacts/comparator-confidence-tier-policy.md`
- `docs/work-packages/20260520-obs01-observability-subsystem-foundation/`

## Phase Plan
### Phase 0 - Surface Inventory and Constraint Audit
- Enumerate every `active` parser surface and contract status.
- Extract explicit inter-surface dependencies and known `HOLD` gap risk markers.
- Capture assumptions and unresolved authority gaps with evidence tags.

### Phase 1 - Prioritization Model
- Define weighted prioritization criteria (critical-path value,
  dependency-centrality, risk/uncertainty, observability leverage,
  compatibility burden).
- Score all surfaces and produce ranked order with rationale.

### Phase 2 - Wave Sequencing and Acceptance Gates
- Group ranked surfaces into implementation waves.
- Define per-wave acceptance checks, required invariants/guards, and
  readiness-to-start conditions.
- Define minimum viable parser stack for first end-to-end confidence signal.

### Phase 3 - Review, Disposition, and Queue Finalization
- Run independent reviewer A/B passes.
- Publish disposition resolving all findings.
- Run verifier A/B passes and finalize GO/HOLD recommendation.
- Publish follow-on implementation work-package queue.

## Exit Criteria
- Every `active` surface in the registry is scored and assigned to an
  implementation wave.
- A deterministic first-implementation critical path is documented.
- Wave-level acceptance criteria are explicit and testable.
- Reviewer and verifier artifacts are complete with no unresolved
  high-severity findings.
- Follow-on implementation work-package queue is actionable and ordered.

## Security Impact and Review Gate
- `security_impact`: `none`
- `dedicated_security_review_required`: `no`
- Rationale: planning/documentation package only.
