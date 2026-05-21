# 20260521-inimpl17-wave2-sidecar-parser-integration-001

## Status
- `state`: active
- `date`: 2026-05-21
- `timezone`: UTC

## Objective
Integrate all Wave 2 worker-package outputs from worktrees into mainline and
close the Wave 2 promotion gates.

## Why This Package Exists
Wave 2 sidecar parser implementation is intentionally parallelized; integration
requires explicit sequencing, conflict handling, and final quality-gate
evidence before promotion.

## Scope
### Included
- Consume worker handoffs from `INIMPL11`..`INIMPL16`.
- Integrate/cherry-pick worker changes in defined order.
- Resolve conflicts while preserving per-surface ownership intent.
- Run Wave 2 global gates (`fmt`, `clippy`, `test`, `deny`) and parser
  acceptance checks.
- Publish integration report and disposition.

### Explicitly Out of Scope
- Wave 3 parser implementation.
- Watershed parser implementation.

## Deliverables
1. Integration report (canonical):
   - `docs/planning/wave2-parser-integration-report.md`
2. Merge/conflict log:
   - `artifacts/merge-conflict-log.md`
3. Gate evidence summary:
   - `artifacts/wave2-gate-evidence.md`
4. Closeout disposition:
   - `artifacts/inimpl17_disposition.md`
5. Review and verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/home/workdir/openWEPP/docs/planning/wave2-parser-worktree-execution-plan.md`
- `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl11-implement-sc-infile-pmetpara-parser-001/`
- `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl12-implement-sc-infile-irrigation-depletion-parser-001/`
- `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl13-implement-sc-infile-irrigation-fixeddate-parser-001/`
- `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl14-implement-sc-infile-frost-parser-001/`
- `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl15-implement-sc-infile-snow-parser-001/`
- `/home/workdir/openWEPP/docs/work-packages/20260521-inimpl16-implement-sc-infile-weppui-parser-001/`

## Integration Order
1. pmetpara parser (`INIMPL11`)
2. irrigation-depletion parser (`INIMPL12`)
3. irrigation-fixeddate parser (`INIMPL13`)
4. frost parser (`INIMPL14`)
5. snow parser (`INIMPL15`)
6. wepp-ui parser (`INIMPL16`)

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
- Wave 2 global gates pass with recorded evidence.
- No unresolved high-severity review findings.

## Security Impact and Review Gate
- `security_impact`: `none`
- `dedicated_security_review_required`: `no`
- Rationale: parser integration only.
