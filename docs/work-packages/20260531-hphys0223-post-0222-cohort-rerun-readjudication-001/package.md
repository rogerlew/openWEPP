# 20260531-hphys0223-post-0222-cohort-rerun-readjudication-001

## Status
- state: completed
- date: 2026-05-31
- timezone: America/Los_Angeles
- decision: HOLD

## Objective
Execute the immediate follow-on rerun/readjudication requested by the
HPHYS0222 review: rerun the `unpalatable-rind` 39-hillslope cohort after
commit `2694f9b`, regenerate semantic parity summaries, and measure residual
movement for monitored families.

## Why This Package Exists
HPHYS0222 changed WB19 branch behavior for `solwpv >= 2006` soils, but that
package intentionally did not rerun the 39-hillslope cohort. This package
closes that measurement gap so downstream remediation attribution remains
defensible.

## Scope
### Included
- Diagnostics-only rerun of `H1..H39` using current openWEPP binary.
- Semantic comparator rerun and summary aggregation.
- Residual-family delta assessment versus HPHYS0221.
- Artifact/disposition publication.

### Explicitly Out of Scope
- Production Rust/kernel edits.
- Contract or external-authority governance edits.
- Watershed reruns.

## Deliverables
1. `artifacts/hphys0223-implementation-and-test-evidence.md`
2. `artifacts/hphys0223-residual-gap-matrix.md`
3. `artifacts/gate-results.md`
4. `artifacts/hphys0223_disposition.md`
5. `artifacts/worker-handoff.md`
6. `artifacts/owned-file-manifest.md`
7. `artifacts/review_agent_a.md`
8. `artifacts/review_agent_b.md`
9. `artifacts/verification_agent_a.md`
10. `artifacts/verification_agent_b.md`

## Dependencies
- `/workdir/openWEPP/AGENTS.md`
- `/workdir/openWEPP/docs/codex_exec_plans.md`
- `/workdir/openWEPP/docs/work-packages/README.md`
- `/workdir/openWEPP/tools/legacy_comparison_suite/semantic_hillslope_wat_compare.py`
- `/tmp/hphys0221_20260531T141839Z/parity/reports/hillslope_semantic_summary.json`
- `/tmp/unpalatable_parity_20260529T192707Z/reports/hillslope/baseline_partitions/`

## Intended Write Set
- `docs/work-packages/README.md`
- `docs/work-packages/20260531-hphys0223-post-0222-cohort-rerun-readjudication-001/**`

## Phase Plan
### Phase A - Rerun
- Execute hillslope batch rerun for `H1..H39`.

### Phase B - Semantic compare
- Recompute semantic reports with valid row-key alignment.

### Phase C - Readjudication
- Aggregate monitored-family metrics and compare to HPHYS0221 baseline.

### Phase D - Closeout
- Publish evidence artifacts and disposition.

## Exit Criteria
- New rerun summaries exist and are valid (`common_row_count > 0`).
- Delta assessment versus HPHYS0221 is documented.
- HOLD/GO decision is explicitly justified.

## Truthfulness Labeling Requirement
Artifacts must label evidence as `Static:` and/or `Ran:`.

## Security Impact and Review Gate
- security_impact: low
- dedicated_security_review_required: no
- rationale: diagnostics and documentation only.
