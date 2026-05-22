# 20260522-inimpl23-wave4-worktree-orchestration-001

## Status
- state: active
- date: 2026-05-22
- timezone: UTC

## Objective
Create the shared Wave 4 watershed-sidecar parser scaffold and the definitive
worktree/ownership plan for concurrent execution across all Wave 4 surfaces.

## Why This Package Exists
Wave 4 parser implementation is decomposed into multiple parallel worker
packages. Shared governance must be published first to avoid write overlap,
merge churn, and ambiguous sequencing.

`ARCH13` ratified `W4DR-001..012`; this package converts those ratified
decisions into executable ownership, sequencing, and gate policy for code
implementation.

## Scope
### Included
- Establish shared Wave 4 parser workspace/module baseline for worker branches.
- Define worktree branch registry and path conventions for `INIMPL24..29`.
- Define disjoint write ownership per Wave 4 worker package.
- Define integration/cherry-pick order and conflict policy for `INIMPL30`.
- Define Wave 4 promotion criteria and cross-package quality gates.
- Encode required strict/compatibility checks implied by `W4DR-001..012`.

### Explicitly Out of Scope
- Surface-specific parser implementation for Wave 4 sidecars.
- Non-Wave-4 parser or kernel implementation.

## Deliverables
1. Worktree execution plan (canonical):
   - `docs/planning/wave4-parser-worktree-execution-plan.md`
2. Ownership manifest:
   - `artifacts/worktree-ownership-manifest.md`
3. Branch/worktree registry:
   - `artifacts/worktree-branch-registry.md`
4. Integration sequencing playbook:
   - `artifacts/wave4-integration-sequence.md`
5. Closeout disposition:
   - `artifacts/inimpl23_disposition.md`
6. Review and verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/home/workdir/openWEPP/docs/planning/parser-implementation-order.md`
- `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl01-prioritize-parser-implementation-order/artifacts/parser-implementation-wave-plan.md`
- `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl22-wave3-core-parser-integration-001/artifacts/inimpl22_disposition.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-arch13-wave4-hold-ratification-checklist-001/artifacts/wave4-hold-ratification-checklist.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-arch13-wave4-hold-ratification-checklist-001/artifacts/wave4-kickoff-acceptance-criteria.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-CHANINP-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-TC-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-GWCOEFF-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-PHOSPHORUS-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-TCR-001.md`
- `/home/workdir/openWEPP/docs/specifications/science-contracts/contracts/SC-INFILE-LCWB-001.md`

## Phase Plan
### Phase 0 - Governance Freeze
- Confirm Wave 4 surface set and dependency graph authority.
- Capture prerequisite status for Wave 3 integration and Wave 4 ratification.

### Phase 1 - Worktree Governance
- Define branch naming, worktree paths, and per-package ownership boundaries.
- Define prohibited overlap paths and conflict escalation policy.

### Phase 2 - Integration Governance
- Define merge/cherry-pick order for `INIMPL24..29` into `INIMPL30`.
- Define blocker criteria, rollback policy, and Wave 4 GO thresholds.

### Phase 3 - Review and Verification
- Dual-agent review/disposition/verification over governance artifacts.

## Exit Criteria
- Canonical Wave 4 worktree plan exists and is actionable.
- Worker package ownership is disjoint and unambiguous.
- Integration sequencing and gates are explicit.
- Wave 4 strict/compat branch checks are mapped to ratified `W4DR` decisions.
- No unresolved high-severity review findings.

## Security Impact and Review Gate
- security_impact: none
- dedicated_security_review_required: no
- Rationale: orchestration/governance docs only.
