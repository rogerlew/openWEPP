# 20260521-inimpl02-wave1-worktree-orchestration-001

## Status
- `state`: active
- `date`: 2026-05-21
- `timezone`: UTC

## Objective
Create the shared Wave 1 parser implementation scaffold and the definitive
worktree/ownership plan for parallel agent execution.

## Why This Package Exists
Wave 1 parser implementation is decomposed into parallel worker packages. To
avoid merge churn and hidden coupling, shared scaffold and worktree governance
must be established first.

## Scope
### Included
- Establish shared parser crate/workspace baseline for Wave 1 worker branches.
- Define worktree branch registry and path conventions.
- Define disjoint write ownership for each worker package.
- Define integration/cherry-pick order and conflict policy.
- Define cross-package quality gates and Wave 1 promotion criteria.

### Explicitly Out of Scope
- Surface-specific parser implementation for `.slp`, `.sol`, `.cli`, `.man`.
- Watershed parser implementation.

## Deliverables
1. Worktree execution plan (canonical):
   - `docs/planning/wave1-parser-worktree-execution-plan.md`
2. Ownership manifest:
   - `artifacts/worktree-ownership-manifest.md`
3. Branch/worktree registry:
   - `artifacts/worktree-branch-registry.md`
4. Integration sequencing playbook:
   - `artifacts/wave1-integration-sequence.md`
5. Closeout disposition:
   - `artifacts/inimpl02_disposition.md`
6. Review and verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `docs/planning/parser-implementation-order.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-SLOPE-001.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-SOIL-001.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-CLIMATE-001.md`
- `docs/specifications/science-contracts/contracts/SC-INFILE-MANAGEMENT-001.md`
- `docs/work-packages/20260521-inimpl01-prioritize-parser-implementation-order/`

## Phase Plan
### Phase 0 - Scaffold Audit
- Confirm baseline workspace/crate layout and shared parser interfaces.
- Capture assumptions as evidence-tagged notes.

### Phase 1 - Worktree Governance
- Define branch naming, worktree paths, and per-package ownership rules.
- Define prohibited overlap paths and conflict resolution process.

### Phase 2 - Integration Governance
- Define merge/cherry-pick order and acceptance gates.
- Define blocker criteria and rollback policy.

### Phase 3 - Review and Verification
- Dual-agent review/disposition/verification over governance artifacts.

## Exit Criteria
- Canonical worktree plan exists and is actionable.
- Worker package ownership is disjoint and unambiguous.
- Integration sequencing and gates are explicit.
- No unresolved high-severity review findings.

## Security Impact and Review Gate
- `security_impact`: `none`
- `dedicated_security_review_required`: `no`
- Rationale: orchestration/governance docs only.
