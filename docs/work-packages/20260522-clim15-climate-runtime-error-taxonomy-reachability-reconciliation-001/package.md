# 20260522-clim15-climate-runtime-error-taxonomy-reachability-reconciliation-001

## Status
- state: queued
- date: 2026-05-22
- timezone: UTC

## Objective
Reconcile climate runtime guard taxonomy with reachable code paths, including
cleanup/redesign of unreachable or misnamed error variants.

## Why This Package Exists
CLIM04 accepted review finding `CLIM04-RVW-006` identified unreachable taxonomy
branch `CLIM-RUNTIME-E-010` and naming inconsistency in guard semantics. This
package closes taxonomy correctness and test-validity gaps.

## Scope
### Included
- Audit climate runtime guard graph vs emitted error variants.
- Remove or redesign unreachable/misnamed variants with stable code/message
  mapping.
- Update tests so taxonomy coverage reflects real guard behavior.

### Explicitly Out of Scope
- Ownership/duplication closure in CLIM11/CLIM12.
- Governance normalization in CLIM16.

## Deliverables
1. Taxonomy reachability report:
   - `artifacts/climate-runtime-error-taxonomy-reachability.md`
2. Guard-path evidence artifact:
   - `artifacts/guard-path-taxonomy-evidence.md`
3. Package governance artifacts:
   - `artifacts/worker-handoff.md`
   - `artifacts/owned-file-manifest.md`
   - `artifacts/gate-results.md`
   - `artifacts/clim15_disposition.md`
4. Dual review/verification artifacts:
   - `artifacts/review_agent_a.md`
   - `artifacts/review_agent_b.md`
   - `artifacts/verification_agent_a.md`
   - `artifacts/verification_agent_b.md`

## Dependencies
- `/home/workdir/openWEPP/AGENTS.md`
- `/home/workdir/openWEPP/docs/work-packages/20260522-clim12-shared-climate-runtime-adapter-extraction-001/`
- `/home/workdir/openWEPP/docs/work-packages/20260522-clim14-runtime-breakpoint-cardinality-policy-closure-001/`
- `/home/workdir/openWEPP/crates/openwepp-hillslope-orchestrator/src/runtime_inputs.rs`
- `/home/workdir/openWEPP/crates/openwepp-watershed-orchestrator/src/runtime_inputs.rs`

## Intended Write Set
- `crates/openwepp-hillslope-orchestrator/**`
- `crates/openwepp-watershed-orchestrator/**`
- `tests/integration/**`
- `docs/work-packages/20260522-clim15-climate-runtime-error-taxonomy-reachability-reconciliation-001/**`

## Phase Plan
### Phase 0 - Intake
- Confirm current taxonomy map and reachable guard paths.

### Phase 1 - Taxonomy Reconciliation
- Implement taxonomy cleanup/redesign and message/code alignment.

### Phase 2 - Verification
- Add tests proving only reachable guard paths emit taxonomy variants.

### Phase 3 - Disposition
- Run required gates and finalize disposition artifacts.

## Exit Criteria
- Taxonomy variants are reachable, correctly named, and test-backed.
- No enum-only synthetic coverage is used as guard-path closure evidence.
- Artifacts include clear evidence-mode labeling (`Static:` vs `Ran:`).
