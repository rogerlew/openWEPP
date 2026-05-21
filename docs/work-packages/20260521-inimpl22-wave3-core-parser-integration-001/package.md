# 20260521-inimpl22-wave3-core-parser-integration-001

## Status
- state: active
- date: 2026-05-21
- timezone: UTC

## Objective
Integrate all Wave 3 worker-package outputs from worktrees into mainline and
close the Wave 3 promotion gates.

## Why This Package Exists
Wave 3 watershed-core parser implementation is intentionally parallelized;
integration requires explicit sequencing, conflict handling, and final
quality-gate evidence before promotion.

## Scope
### Included
- Consume worker handoffs from INIMPL19..INIMPL21.
- Integrate/cherry-pick worker changes in defined order.
- Resolve conflicts while preserving per-surface ownership intent.
- Run Wave 3 global gates (fmt, clippy, test, deny) and parser
  acceptance checks.
- Publish integration report and disposition.

### Explicitly Out of Scope
- Wave 4 parser implementation.
- Watershed sidecar parser implementation.

## Deliverables
1. Integration report (canonical):
   - docs/planning/wave3-parser-integration-report.md
2. Merge/conflict log:
   - artifacts/merge-conflict-log.md
3. Gate evidence summary:
   - artifacts/wave3-gate-evidence.md
4. Closeout disposition:
   - artifacts/inimpl22_disposition.md
5. Review and verification artifacts:
   - artifacts/review_agent_a.md
   - artifacts/review_agent_b.md
   - artifacts/verification_agent_a.md
   - artifacts/verification_agent_b.md

## Dependencies
- /home/workdir/openWEPP/docs/planning/wave3-parser-worktree-execution-plan.md
- /home/workdir/openWEPP/docs/work-packages/20260521-inimpl19-implement-sc-infile-watershed-structure-parser-001/
- /home/workdir/openWEPP/docs/work-packages/20260521-inimpl20-implement-sc-infile-watershed-channel-parser-001/
- /home/workdir/openWEPP/docs/work-packages/20260521-inimpl21-implement-sc-infile-watershed-impoundment-parser-001/

## Integration Order
1. watershed-structure parser (INIMPL19)
2. watershed-channel parser (INIMPL20)
3. watershed-impoundment parser (INIMPL21)

## Phase Plan
### Phase 0 - Intake
- Validate worker handoff completeness and owned-file manifests.

### Phase 1 - Integration
- Apply worker changes in integration order.
- Resolve conflicts with explicit log entries.

### Phase 2 - Validation Gates
- Run workspace quality gates and parser acceptance tests.
- Record Ran evidence.

### Phase 3 - Closeout
- Publish integration report and disposition.
- Run dual review/disposition/verification.

## Exit Criteria
- All worker changes are integrated on mainline.
- Wave 3 global gates pass with recorded evidence.
- No unresolved high-severity review findings.

## Security Impact and Review Gate
- security_impact: none
- dedicated_security_review_required: no
- Rationale: parser integration only.
