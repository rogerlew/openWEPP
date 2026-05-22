# 20260522-inimpl30-wave4-sidecar-parser-integration-001

## Status
- state: active
- date: 2026-05-22
- timezone: UTC

## Objective
Integrate all Wave 4 worker-package outputs from worktrees into mainline and
close the Wave 4 promotion gates.

## Why This Package Exists
Wave 4 watershed-sidecar parser implementation is intentionally parallelized;
integration requires explicit sequencing, conflict handling, and final
quality-gate evidence before promotion.

## Scope
### Included
- Consume worker handoffs from INIMPL24..INIMPL29.
- Integrate/cherry-pick worker changes in defined order.
- Resolve conflicts while preserving per-surface ownership intent.
- Run Wave 4 global gates (fmt, clippy, test, deny) and parser
  acceptance checks.
- Validate and report W4DR-001..012 implementation evidence closure.
- Publish integration report and disposition.

### Explicitly Out of Scope
- New Wave 4 parser implementation not already in worker handoffs.
- Non-parser kernel/orchestrator implementation.

## Deliverables
1. Integration report (canonical):
   - docs/planning/wave4-parser-integration-report.md
2. Merge/conflict log:
   - artifacts/merge-conflict-log.md
3. Gate evidence summary:
   - artifacts/wave4-gate-evidence.md
4. W4DR closure report:
   - artifacts/w4dr-closure-report.md
5. Closeout disposition:
   - artifacts/inimpl30_disposition.md
6. Review and verification artifacts:
   - artifacts/review_agent_a.md
   - artifacts/review_agent_b.md
   - artifacts/verification_agent_a.md
   - artifacts/verification_agent_b.md

## Dependencies
- /home/workdir/openWEPP/docs/planning/wave4-parser-worktree-execution-plan.md
- /home/workdir/openWEPP/docs/work-packages/20260522-inimpl23-wave4-worktree-orchestration-001/
- /home/workdir/openWEPP/docs/work-packages/20260522-inimpl24-implement-sc-infile-chaninp-parser-001/
- /home/workdir/openWEPP/docs/work-packages/20260522-inimpl25-implement-sc-infile-tc-parser-001/
- /home/workdir/openWEPP/docs/work-packages/20260522-inimpl26-implement-sc-infile-gwcoeff-parser-001/
- /home/workdir/openWEPP/docs/work-packages/20260522-inimpl27-implement-sc-infile-tcr-parser-001/
- /home/workdir/openWEPP/docs/work-packages/20260522-inimpl28-implement-sc-infile-phosphorus-parser-001/
- /home/workdir/openWEPP/docs/work-packages/20260522-inimpl29-implement-sc-infile-lcwb-parser-001/

## Integration Order
1. chaninp parser (INIMPL24)
2. tc parser (INIMPL25)
3. gwcoeff parser (INIMPL26)
4. tcr parser (INIMPL27)
5. phosphorus parser (INIMPL28)
6. lcwb parser (INIMPL29)

## Phase Plan
### Phase 0 - Intake
- Validate worker handoff completeness and owned-file manifests.

### Phase 1 - Integration
- Apply worker changes in integration order.
- Resolve conflicts with explicit log entries.

### Phase 2 - Validation Gates
- Run workspace quality gates and parser acceptance tests.
- Verify W4DR closure evidence and record Ran evidence.

### Phase 3 - Closeout
- Publish integration report and disposition.
- Run dual review/disposition/verification.

## Exit Criteria
- All worker changes are integrated on mainline.
- Wave 4 global gates pass with recorded evidence.
- W4DR-001..012 closure evidence is published.
- No unresolved high-severity review findings.

## Security Impact and Review Gate
- security_impact: none
- dedicated_security_review_required: no
- Rationale: parser integration only.
