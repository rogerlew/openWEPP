# 20260521-inimpl07-wave1-core-parser-integration-001

## Status
- `state`: active
- `date`: 2026-05-21
- `timezone`: UTC

## Objective
Integrate all Wave 1 worker-package outputs from worktrees into mainline and
close the Wave 1 promotion gates.

## Why This Package Exists
Wave 1 is intentionally parallelized; integration requires explicit sequencing,
conflict handling, and final quality-gate evidence before promotion.

## Scope
### Included
- Consume worker handoffs from `INIMPL03`..`INIMPL06`.
- Integrate/cherry-pick worker changes in defined order.
- Resolve conflicts while preserving per-surface ownership intent.
- Run Wave 1 global gates (`fmt`, `clippy`, `test`, `deny`) and parser
  acceptance checks.
- Publish integration report and disposition.

### Explicitly Out of Scope
- Wave 2 parser implementation.
- Watershed parser implementation.

## Deliverables
1. Integration report (canonical):
   - `docs/planning/wave1-parser-integration-report.md`
2. Merge/conflict log:
   - `artifacts/merge-conflict-log.md`
3. Gate evidence summary:
   - `artifacts/wave1-gate-evidence.md`
4. Closeout disposition:
   - `artifacts/inimpl07_disposition.md`
5. Review and verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `docs/planning/wave1-parser-worktree-execution-plan.md`
- `docs/work-packages/20260521-inimpl03-implement-sc-infile-slope-parser-001/`
- `docs/work-packages/20260521-inimpl04-implement-sc-infile-soil-parser-001/`
- `docs/work-packages/20260521-inimpl05-implement-sc-infile-climate-parser-001/`
- `docs/work-packages/20260521-inimpl06-implement-sc-infile-management-parser-001/`

## Integration Order
1. slope parser (`INIMPL03`)
2. soil parser (`INIMPL04`)
3. climate parser (`INIMPL05`)
4. management parser (`INIMPL06`)

## Phase Plan
### Phase 0 - Intake
- Validate worker handoff completeness and owned-file manifests.

### Phase 1 - Integration
- Apply worker changes in integration order.
- Resolve conflicts with explicit log entries.

### Phase 2 - Validation Gates
- Run workspace quality gates and parser acceptance tests.
- Record `Ran` evidence.

### Phase 3 - Closeout
- Publish integration report and disposition.
- Run dual review/disposition/verification.

## Exit Criteria
- All worker changes are integrated on mainline.
- Wave 1 global gates pass with recorded evidence.
- No unresolved high-severity review findings.

## Security Impact and Review Gate
- `security_impact`: `none`
- `dedicated_security_review_required`: `no`
- Rationale: parser integration only.
