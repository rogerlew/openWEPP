# 20260521-inimpl18-wave3-worktree-orchestration-001

## Status
- state: active
- date: 2026-05-21
- timezone: UTC

## Objective
Create the shared Wave 3 watershed-core parser scaffold and the definitive
worktree/ownership plan for concurrent execution across Wave 3 surfaces.

## Why This Package Exists
Wave 3 watershed-core parser implementation is decomposed into parallel worker
packages. Shared governance must be published first to avoid write overlap,
merge churn, and ambiguous sequencing.

## Scope
### Included
- Establish shared Wave 3 parser workspace/module baseline for worker branches.
- Define worktree branch registry and path conventions for INIMPL19..21.
- Define disjoint write ownership per Wave 3 worker package.
- Define integration/cherry-pick order and conflict policy for INIMPL22.
- Define Wave 3 promotion criteria and cross-package quality gates.

### Explicitly Out of Scope
- Surface-specific parser implementation for Wave 3 surfaces.
- Wave 4 watershed sidecar parser implementation.

## Deliverables
1. Worktree execution plan (canonical):
   - docs/planning/wave3-parser-worktree-execution-plan.md
2. Ownership manifest:
   - artifacts/worktree-ownership-manifest.md
3. Branch/worktree registry:
   - artifacts/worktree-branch-registry.md
4. Integration sequencing playbook:
   - artifacts/wave3-integration-sequence.md
5. Closeout disposition:
   - artifacts/inimpl18_disposition.md
6. Review and verification artifacts:
   - artifacts/review_agent_a.md
   - artifacts/review_agent_b.md
   - artifacts/verification_agent_a.md
   - artifacts/verification_agent_b.md

## Dependencies
- /home/workdir/openWEPP/docs/planning/parser-implementation-order.md
- /home/workdir/openWEPP/docs/work-packages/20260521-inimpl01-prioritize-parser-implementation-order/artifacts/parser-implementation-wave-plan.md
- /home/workdir/openWEPP/docs/work-packages/20260521-inimpl01-prioritize-parser-implementation-order/artifacts/parser-implementation-priority-matrix.csv
- /home/workdir/openWEPP/docs/work-packages/20260521-inimpl17-wave2-sidecar-parser-integration-001/
- /home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-STRUCTURE-001.md
- /home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-CHANNEL-001.md
- /home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-WATERSHED-IMPOUNDMENT-001.md

## Phase Plan
### Phase 0 - Governance Freeze
- Confirm Wave 3 surface set and dependency graph authority.
- Capture prerequisite status for Wave 2 integration and watershed contract gates.

### Phase 1 - Worktree Governance
- Define branch naming, worktree paths, and per-package ownership boundaries.
- Define prohibited overlap paths and conflict escalation policy.

### Phase 2 - Integration Governance
- Define merge/cherry-pick order for INIMPL19..21 into INIMPL22.
- Define blocker criteria, rollback policy, and Wave 3 GO thresholds.

### Phase 3 - Review and Verification
- Dual-agent review/disposition/verification over governance artifacts.

## Exit Criteria
- Canonical Wave 3 worktree plan exists and is actionable.
- Worker package ownership is disjoint and unambiguous.
- Integration sequencing and gates are explicit.
- No unresolved high-severity review findings.

## Security Impact and Review Gate
- security_impact: none
- dedicated_security_review_required: no
- Rationale: orchestration/governance docs only.
